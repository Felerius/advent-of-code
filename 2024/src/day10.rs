use std::{array, collections::VecDeque};

use anyhow::Result;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut cnt = vec![vec![0; width]; height];
    let mut by_height: [_; 9] = array::from_fn(|_| vec![]);
    for (y, row) in grid.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            if h == b'9' {
                cnt[y][x] = 1;
            } else {
                by_height[usize::from(h - b'0')].push((y, x));
            }
        }
    }

    let mut part2 = 0;
    for (h, pos) in by_height.iter().enumerate().rev() {
        for &(y, x) in pos {
            for (y2, x2) in neighbors(y, x, &grid) {
                cnt[y][x] += cnt[y2][x2];
            }
            if h == 0 {
                part2 += cnt[y][x];
            }
        }
    }

    let mut part1 = 0;
    let mut queue = VecDeque::new();
    let mut seen = vec![vec![usize::MAX; width]; height];
    for (i, &(y0, x0)) in by_height[0].iter().enumerate() {
        queue.push_back((y0, x0));
        seen[y0][x0] = i;
        while let Some((y, x)) = queue.pop_front() {
            if grid[y][x] == b'9' {
                part1 += 1;
            } else {
                for (y2, x2) in neighbors(y, x, &grid) {
                    if seen[y2][x2] != i {
                        seen[y2][x2] = i;
                        queue.push_back((y2, x2));
                    }
                }
            }
        }
    }

    Ok((part1, part2))
}

fn neighbors<'a>(
    y: usize,
    x: usize,
    grid: &'a [&'a [u8]],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let candidates = [
        y.checked_sub(1).map(|yi| (yi, x)),
        x.checked_sub(1).map(|xi| (y, xi)),
        (y + 1 < grid.len()).then(|| (y + 1, x)),
        (x + 1 < grid[0].len()).then(|| (y, x + 1)),
    ];

    candidates
        .into_iter()
        .flatten()
        .filter(move |&(y2, x2)| grid[y2][x2] == grid[y][x] + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (36, 81));
    }
}
