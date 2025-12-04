use std::collections::VecDeque;

use itertools::iproduct;
use ndarray::{Array2, s};
use register::register;
use utils::grid;

#[register]
fn run(input: &str) -> (usize, usize) {
    let grid = grid::from_lines(input);
    let mut num_neighbors = Array2::from_elem(grid.dim(), 255_u8);
    for (pos, &cell) in grid.indexed_iter() {
        if cell == b'@' {
            num_neighbors[pos] = neighbors(pos, grid.dim())
                .filter(|&pos2| grid[pos2] == b'@')
                .count() as u8;
        }
    }

    let mut ready: VecDeque<_> = num_neighbors
        .indexed_iter()
        .filter(|&(_, &cnt)| cnt < 4)
        .map(|(pos, _)| pos)
        .collect();

    let part1 = ready.len();
    let mut part2 = 0;
    while let Some(pos) = ready.pop_front() {
        part2 += 1;
        for pos2 in neighbors(pos, grid.dim()) {
            num_neighbors[pos2] -= 1;
            if num_neighbors[pos2] == 3 {
                ready.push_back(pos2);
            }
        }
    }

    (part1, part2)
}

#[expect(dead_code, reason = "alternative implementation")]
fn run_slow(input: &str) -> (usize, usize) {
    let height = input.lines().count();
    let width = input.lines().next().map_or(0, str::len);
    let mut grid = Array2::from_elem((height + 2, width + 2), b'.');
    let inner_rows = grid.rows_mut().into_iter().skip(1);
    for (line, mut row) in input.lines().zip(inner_rows) {
        for (x, byte) in line.bytes().enumerate() {
            row[x + 1] = byte;
        }
    }

    let part1 = iproduct!(1..=height, 1..=width)
        .filter(|&(y, x)| grid[(y, x)] == b'@')
        .filter(|&(y, x)| {
            let rolls = grid
                .slice(s![y - 1..=y + 1, x - 1..=x + 1])
                .iter()
                .filter(|&&b| b == b'@')
                .count();
            rolls <= 4
        })
        .count();

    let mut part2 = 0;
    loop {
        let part2_start = part2;
        for y in 1..=height {
            for x in 1..=width {
                if grid[(y, x)] != b'@' {
                    continue;
                }

                let rolls = grid
                    .slice(s![y - 1..=y + 1, x - 1..=x + 1])
                    .iter()
                    .filter(|&&b| b == b'@')
                    .count();
                if rolls <= 4 {
                    grid[(y, x)] = b'.';
                    part2 += 1;
                }
            }
        }

        if part2 == part2_start {
            break;
        }
    }

    (part1, part2)
}

fn neighbors(
    (y, x): (usize, usize),
    (height, width): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let min_y = y.saturating_sub(1);
    let max_y = (y + 2).min(height);
    let min_x = x.saturating_sub(1);
    let max_x = (x + 2).min(width);
    iproduct!(min_y..max_y, min_x..max_x).filter(move |&pos| pos != (y, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (13, 43));
    }
}
