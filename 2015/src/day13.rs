use anyhow::{Context, Result};
use arrayvec::ArrayVec;
use itertools::Itertools;
use register::register;
use utils::num::bits;

const MAX: usize = 8;

#[register]
fn run(input: &str) -> Result<(i64, i64)> {
    let (n, matrix) = parse(input)?;
    assert!(n > 2);
    let mut dp = [[[i64::MIN; MAX]; MAX]; 1 << MAX];
    for i in 0..n {
        dp[1 << i][i][i] = 0;
    }

    for m in 3_usize..(1 << n) {
        if m.count_ones() < 2 {
            continue;
        }

        for last in bits(m) {
            let m2 = m ^ (1 << last);
            for first in bits(m2) {
                for second_last in bits(m2) {
                    dp[m][first][last] = dp[m][first][last].max(
                        dp[m2][first][second_last]
                            .saturating_add(i64::from(matrix[second_last][last])),
                    );
                }
            }
        }
    }

    let all = (1 << n) - 1;
    let solution = (0..n).cartesian_product(0..n).fold(
        (i64::MIN, i64::MIN),
        |(part1, part2), (first, last)| {
            let val1 = dp[all][first][last].saturating_add(i64::from(matrix[last][first]));
            let val2 = dp[all][last][first];
            (part1.max(val1), part2.max(val2))
        },
    );
    Ok(solution)
}

fn parse<'a>(input: &'a str) -> Result<(usize, [[i32; MAX]; MAX])> {
    let mut matrix = [[0; MAX]; MAX];
    let mut names = ArrayVec::<_, MAX>::new();
    let mut name_to_index = |name: &'a str| {
        if let Some(index) = names.iter().position(|&n| n == name) {
            index
        } else {
            names.push(name);
            names.len() - 1
        }
    };

    for line in input.lines() {
        let (from, tail) = line.split_once(' ').context("invalid input")?;
        let from = name_to_index(from);
        let (mid, to) = tail.rsplit_once(' ').context("invalid input")?;
        let to = name_to_index(to.trim_end_matches('.'));
        let negative = mid[6..].starts_with("lose");
        let num: i32 = mid[11..]
            .split_once(' ')
            .context("invalid input")?
            .0
            .parse()?;
        let num = if negative { -num } else { num };
        matrix[from][to] += num;
        matrix[to][from] += num;
    }

    Ok((names.len(), matrix))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).unwrap().0, 330);
    }
}
