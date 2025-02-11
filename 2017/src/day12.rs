use anyhow::Result;
use utils::input::Input;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let lines: Vec<_> = input.lines().collect();
    let mut seen = vec![false; lines.len()];

    let mut part1 = 0;
    let mut part2 = 0;
    for v0 in 0..lines.len() {
        if !seen[v0] {
            let size = dfs(v0, &mut seen, &lines)?;
            if v0 == 0 {
                part1 = size;
            }
            part2 += 1;
        }
    }

    Ok((part1, part2))
}

fn dfs(v: usize, seen: &mut [bool], lines: &[&str]) -> Result<usize> {
    seen[v] = true;
    lines[v][6..]
        .unsigned_integers()
        .try_fold(1, |mut cnt, neighbor: usize| {
            if !seen[neighbor] {
                cnt += dfs(neighbor, seen, lines)?;
            }
            Ok(cnt)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (6, 2));
    }
}
