use std::iter;

use itertools::Itertools;

pub(crate) fn run(input: &str) -> (usize, usize) {
    let mut directional_matrices = [[[1; 5]; 5]; 26];
    for i in 1..26 {
        directional_matrices[i] = compute_transition_table(
            DirectionalKey::POSITIONS,
            DirectionalKey::FORBIDDEN_CORNER,
            &directional_matrices[i - 1],
        );
    }
    let numeric_matrix1 = compute_transition_table(
        NumericKey::POSITIONS,
        NumericKey::FORBIDDEN_CORNER,
        &directional_matrices[2],
    );
    let numeric_matrix2 = compute_transition_table(
        NumericKey::POSITIONS,
        NumericKey::FORBIDDEN_CORNER,
        &directional_matrices[25],
    );

    input
        .lines()
        .map(|line| {
            let keys = line.bytes().map(NumericKey::from_char);
            let cmd_len1 = compute_cost(NumericKey::A, keys.clone(), &numeric_matrix1);
            let cmd_len2 = compute_cost(NumericKey::A, keys, &numeric_matrix2);
            let number: usize = line[..line.len() - 1].parse().unwrap();
            (number * cmd_len1, number * cmd_len2)
        })
        .fold((0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
}

fn compute_transition_table<const N: usize>(
    pos: [Point; N],
    forbidden_corner: Point,
    base: &[[usize; 5]; 5],
) -> [[usize; N]; N] {
    pos.map(|from| {
        pos.map(|to| {
            let hor_key = if from.x <= to.x {
                DirectionalKey::Right
            } else {
                DirectionalKey::Left
            };
            let vert_key = if from.y <= to.y {
                DirectionalKey::Down
            } else {
                DirectionalKey::Up
            };

            let hor_dist = from.x.abs_diff(to.x);
            let vert_dist = from.y.abs_diff(to.y);
            let hor_first_corner = Point { x: to.x, y: from.y };
            let vert_first_corner = Point { x: from.x, y: to.y };
            let paths = [
                (
                    hor_first_corner,
                    [
                        (hor_key, hor_dist),
                        (vert_key, vert_dist),
                        (DirectionalKey::A, 1),
                    ],
                ),
                (
                    vert_first_corner,
                    [
                        (vert_key, vert_dist),
                        (hor_key, hor_dist),
                        (DirectionalKey::A, 1),
                    ],
                ),
            ];

            paths
                .into_iter()
                .filter(|&(corner, _)| corner != forbidden_corner)
                .map(|(_, path)| {
                    let expanded_path = path
                        .into_iter()
                        .flat_map(|(key, cnt)| iter::repeat_n(key, cnt));
                    compute_cost(DirectionalKey::A, expanded_path, &base)
                })
                .min()
                .unwrap()
        })
    })
}

fn compute_cost<T: Clone + Into<usize>, const N: usize>(
    start: T,
    path: impl IntoIterator<Item = T>,
    costs: &[[usize; N]; N],
) -> usize {
    itertools::chain!([start], path)
        .tuple_windows()
        .map(|(from, to)| costs[from.into()][to.into()])
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectionalKey {
    Up,
    A,
    Left,
    Down,
    Right,
}

impl DirectionalKey {
    const POSITIONS: [Point; 5] = [
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 1 },
    ];

    const FORBIDDEN_CORNER: Point = Point { x: 0, y: 0 };
}

impl Into<usize> for DirectionalKey {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumericKey {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}

impl NumericKey {
    const POSITIONS: [Point; 11] = [
        Point { x: 0, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 2 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 1 },
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 1, y: 3 },
        Point { x: 2, y: 3 },
    ];

    const FORBIDDEN_CORNER: Point = Point { x: 0, y: 3 };

    fn from_char(c: u8) -> Self {
        match c {
            b'0' => Self::Zero,
            b'1' => Self::One,
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'A' => Self::A,
            _ => panic!("Invalid numeric key: {:?}", char::from(c)),
        }
    }
}

impl Into<usize> for NumericKey {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "029A\n980A\n179A\n456A\n379A";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).0, 126384);
    }
}
