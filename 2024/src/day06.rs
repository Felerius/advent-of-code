use anyhow::{ensure, Context, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let (y0, x0) = itertools::iproduct!(0..height, 0..width)
        .find(|&(y, x)| grid[y][x] == b'^')
        .context("no start")?;
    let mut seen = vec![vec![0; width]; height];
    let loop1 = simulate(&grid, y0, x0, &mut seen);
    ensure!(!loop1, "input loops already");

    let candidates: Vec<_> = itertools::iproduct!(0..height, 0..width)
        .filter(|&(y, x)| seen[y][x] != 0 && (y, x) != (y0, x0))
        .collect();
    let part1 = candidates.len() + 1;

    let part2 = candidates
        .into_par_iter()
        .map_init(
            || (grid.clone(), vec![vec![0; width]; height]),
            |(grid, seen), (y, x)| {
                for row in &mut *seen {
                    row.fill(0);
                }

                grid[y][x] = b'#';
                let loops = simulate(&grid, y0, x0, seen);
                grid[y][x] = b'.';

                loops
            },
        )
        .filter(|&loops| loops)
        .count();

    Ok((part1, part2))
}

fn simulate(grid: &[Vec<u8>], y0: usize, x0: usize, seen: &mut [Vec<u8>]) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let mut y = y0;
    let mut x = x0;
    let mut direction = Direction::Up;
    loop {
        if seen[y][x] & 1 << direction as u8 != 0 {
            break true;
        }
        seen[y][x] |= 1 << direction as u8;

        let next = direction
            .step(y, x)
            .filter(|&(y2, x2)| y2 < height && x2 < width);
        let Some((y2, x2)) = next else {
            break false;
        };

        if grid[y2][x2] == b'#' {
            direction = direction.rotate_right();
        } else {
            (y, x) = (y2, x2);
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(self, y: usize, x: usize) -> Option<(usize, usize)> {
        match self {
            Self::Up => y.checked_sub(1).map(|yi| (yi, x)),
            Self::Right => Some((y, x + 1)),
            Self::Down => Some((y + 1, x)),
            Self::Left => x.checked_sub(1).map(|xi| (y, xi)),
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (41, 6));
    }
}
