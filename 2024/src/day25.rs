use std::array;

use itertools::Itertools;
use register::register;

#[register]
fn run(input: &str) -> (usize, u8) {
    let mut locks = [[[[[0; 6]; 6]; 6]; 6]; 6];
    let mut keys = Vec::new();
    let line_groups = input
        .lines()
        .map(str::as_bytes)
        .filter(|line| !line.is_empty())
        .tuples();
    for (l1, l2, l3, l4, l5, l6, _) in line_groups {
        let [h1, h2, h3, h4, h5] = array::from_fn(|i| {
            [l2, l3, l4, l5, l6]
                .iter()
                .filter(|line| line[i] == b'#')
                .count()
        });

        if l1 == b"#####" {
            locks[h1][h2][h3][h4][h5] += 1;
        } else {
            keys.push([h1, h2, h3, h4, h5]);
        }
    }

    for (h1, h2, h3, h4, h5) in itertools::iproduct!(1..6, 0..6, 0..6, 0..6, 0..6) {
        locks[h1][h2][h3][h4][h5] += locks[h1 - 1][h2][h3][h4][h5];
    }
    for (h1, h2, h3, h4, h5) in itertools::iproduct!(0..6, 1..6, 0..6, 0..6, 0..6) {
        locks[h1][h2][h3][h4][h5] += locks[h1][h2 - 1][h3][h4][h5];
    }
    for (h1, h2, h3, h4, h5) in itertools::iproduct!(0..6, 0..6, 1..6, 0..6, 0..6) {
        locks[h1][h2][h3][h4][h5] += locks[h1][h2][h3 - 1][h4][h5];
    }
    for (h1, h2, h3, h4, h5) in itertools::iproduct!(0..6, 0..6, 0..6, 1..6, 0..6) {
        locks[h1][h2][h3][h4][h5] += locks[h1][h2][h3][h4 - 1][h5];
    }
    for (h1, h2, h3, h4, h5) in itertools::iproduct!(0..6, 0..6, 0..6, 0..6, 1..6) {
        locks[h1][h2][h3][h4][h5] += locks[h1][h2][h3][h4][h5 - 1];
    }

    let part1 = keys
        .iter()
        .map(|&[h1, h2, h3, h4, h5]| locks[5 - h1][5 - h2][5 - h3][5 - h4][5 - h5])
        .sum();

    (part1, 0)
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
        assert_eq!(run(INPUT).0, 3);
    }
}
