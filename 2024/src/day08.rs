use std::array;

use itertools::Itertools;
use num::Integer;

pub fn run(input: &str) -> (usize, usize) {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let to_grid_pos = |(x, y): (isize, isize)| {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        (x < width && y < height).then_some((x, y))
    };

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

            if let Some((x3, y3)) = to_grid_pos((x2 + dx, y2 + dy)) {
                antinode[y3][x3] |= 1;
            }
            if let Some((x3, y3)) = to_grid_pos((x1 - dx, y1 - dy)) {
                antinode[y3][x3] |= 1;
            }

            let g = dx.gcd(&dy);
            let dx = dx / g;
            let dy = dy / g;

            let antinodes1 = (0..).map_while(|i| to_grid_pos((x1 + dx * i, y1 + dy * i)));
            let antinodes2 = (1..).map_while(|i| to_grid_pos((x1 - dx * i, y1 - dy * i)));
            for (x, y) in antinodes1.chain(antinodes2) {
                antinode[y as usize][x as usize] |= 2;
            }
        }
    }

    antinode
        .iter()
        .flatten()
        .fold((0, 0), |(part1, part2), &b| {
            (part1 + usize::from(b & 1), part2 + usize::from(b & 2 != 0))
        })
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
        assert_eq!(run(INPUT), (14, 34));
    }
}
