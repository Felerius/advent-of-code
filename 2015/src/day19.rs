use std::iter;

use anyhow::{Context, Result};
use itertools::Itertools;
use utils::hash::{FastHashCollectionExt, FastHashMap, FastHashSet};

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut lines = input.lines();
    let input = lines.next_back().context("empty input")?;
    let mut cnf = ChomskyNormalForm {
        rules: Vec::new(),
        names: FastHashMap::new(),
    };
    let mut part1 = FastHashSet::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (from, to) = line.split_once(" => ").context("invalid rule")?;
        let mut start = 0;
        while let Some(off) = input[start..].find(from) {
            let result = format!(
                "{}{}{}",
                &input[..start + off],
                to,
                &input[start + off + from.len()..]
            );
            part1.insert(result);
            start += off + 1;
        }

        let from = cnf.resolve(from.as_bytes());
        let mut to = split_elements(to.as_bytes());
        let (to1, to2) = to
            .next_tuple()
            .context("rule needs to map to at least 2 elements")?;
        let to1 = cnf.resolve(to1);
        let mut to2 = cnf.resolve(to2);
        for to3 in to {
            let to3 = cnf.resolve(to3);
            cnf.rules.push(vec![(to2, to3, false)]);
            to2 = cnf.rules.len() - 1;
        }
        cnf.rules[from].push((to1, to2, true));
    }

    let input: Vec<_> = split_elements(input.as_bytes()).collect();
    let n = input.len();
    let m = cnf.rules.len();
    let mut dp = vec![vec![vec![usize::MAX / 3; m]; n + 1]; n];
    for (i, elem) in input.iter().enumerate() {
        dp[i][i + 1][cnf.resolve(elem)] = 0;
    }

    for w in 2..=n {
        for l in 0..=n - w {
            let r = l + w;
            for (from, rules) in cnf.rules.iter().enumerate() {
                let mut mn = usize::MAX / 3;
                for &(to1, to2, cost) in rules {
                    for mid in l + 1..r {
                        mn = mn.min(usize::from(cost) + dp[l][mid][to1] + dp[mid][r][to2]);
                    }
                }
                dp[l][r][from] = mn;
            }
        }
    }

    let start = cnf.resolve(b"e");
    let part2 = dp[0][n][start];
    Ok((part1.len(), part2))
}

struct ChomskyNormalForm<'a> {
    rules: Vec<Vec<(usize, usize, bool)>>,
    names: FastHashMap<&'a [u8], usize>,
}

impl<'a> ChomskyNormalForm<'a> {
    fn resolve(&mut self, name: &'a [u8]) -> usize {
        *self.names.entry(name).or_insert_with(|| {
            self.rules.push(Vec::new());
            self.rules.len() - 1
        })
    }
}

fn split_elements<'a>(mut input: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    iter::from_fn(move || {
        if let Some((elem, tail)) = input
            .split_at_checked(2)
            .filter(|(elem, _)| elem[1].is_ascii_lowercase())
        {
            input = tail;
            Some(elem)
        } else if let Some((elem, tail)) = input.split_at_checked(1) {
            input = tail;
            Some(elem)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
e => AH
e => AO
H => HO
H => OH
O => HH

AHOH
";

    const INPUT2: &str = "\
e => AH
e => AO
H => HO
H => OH
O => HH

AHOHOHO
";

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(run(INPUT1)?.1, 3);
        assert_eq!(run(INPUT2)?.1, 6);
        Ok(())
    }
}
