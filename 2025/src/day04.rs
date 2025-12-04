use itertools::iproduct;
use ndarray::{Array2, s};
use register::register;

#[register]
fn run(input: &str) -> (usize, usize) {
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
