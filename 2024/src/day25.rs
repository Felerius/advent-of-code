use std::array;

use anyhow::Result;
use itertools::Itertools;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let line_groups = input
        .lines()
        .map(|line| line.as_bytes())
        .filter(|line| !line.is_empty())
        .tuples();
    for (l1, l2, l3, l4, l5, l6, _) in line_groups {
        let heights = parse_heights(&[l2, l3, l4, l5, l6]);
        if l1 == b"#####" {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let part1 = itertools::iproduct!(locks, keys.iter().copied())
        .filter(|&(lock, key)| lock.into_iter().zip(key).all(|(l, k)| l + k <= 5))
        .count();

    Ok((part1, 0))
}

fn parse_heights(lines: &[&[u8]; 5]) -> [u8; 5] {
    array::from_fn(|i| lines.iter().filter(|line| line[i] == b'#').count() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).unwrap().0, 3);
    }
}
