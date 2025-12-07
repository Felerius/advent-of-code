use std::mem;

use register::register;
use utils::grid;

#[register]
fn run(input: &str) -> (usize, u64) {
    let grid = grid::from_lines(input);
    let mut beams = vec![0_u64; grid.ncols()];
    let mut beams_next = beams.clone();
    let mut part1 = 0;
    for row in grid.rows() {
        for (i, &cell) in row.indexed_iter() {
            match cell {
                b'S' => {
                    beams_next[i] = 1;
                }
                b'^' => {
                    beams_next[i - 1] += beams[i];
                    beams_next[i + 1] += beams[i];
                    part1 += usize::from(beams[i] > 0);
                }
                _ => {
                    beams_next[i] += beams[i];
                }
            }
        }

        mem::swap(&mut beams_next, &mut beams);
        beams_next.fill(0);
    }

    (part1, beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (21, 40));
    }
}
