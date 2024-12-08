use std::array;

use anyhow::Result;
use itertools::Itertools;
use num::Integer;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut antennas: [_; 256] = array::from_fn(|_| vec![]);
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != b'.' {
                antennas[usize::from(c)].push((x as isize, y as isize));
            }
        }
    }

    let mut antinode = vec![vec![0_u8; width]; height];
    for pos in &antennas {
        for (&(x1, y1), &(x2, y2)) in pos.iter().tuple_combinations() {
            let dx = x2 - x1;
            let dy = y2 - y1;

            let (x3, y3) = (x2 + dx, y2 + dy);
            if (0..height as isize).contains(&y3) && (0..width as isize).contains(&x3) {
                antinode[y3 as usize][x3 as usize] |= 1;
            }
            let (x4, y4) = (x1 - dx, y1 - dy);
            if (0..height as isize).contains(&y4) && (0..width as isize).contains(&x4) {
                antinode[y4 as usize][x4 as usize] |= 1;
            }

            let g = dx.gcd(&dy);
            let dx = dx / g;
            let dy = dy / g;
            for i in 0.. {
                let (x3, y3) = (x1 + dx * i, y1 + dy * i);
                if (0..height as isize).contains(&y3) && (0..width as isize).contains(&x3) {
                    antinode[y3 as usize][x3 as usize] |= 2;
                } else {
                    break;
                }
            }
            for i in 1.. {
                let (x4, y4) = (x2 - dx * i, y2 - dy * i);
                if (0..height as isize).contains(&y4) && (0..width as isize).contains(&x4) {
                    antinode[y4 as usize][x4 as usize] |= 2;
                } else {
                    break;
                }
            }
        }
    }

    let part1 = antinode.iter().flatten().filter(|&&b| b & 1 != 0).count();
    let part2 = antinode.iter().flatten().filter(|&&b| b & 2 != 0).count();

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (14, 34));
    }
}
