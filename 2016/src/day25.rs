use std::iter;

use utils::input;

pub(crate) fn run(input: &str) -> (usize, u8) {
    let mut lines = input.lines();
    let [a] = input::integers::<usize, 1>(lines.nth(1).unwrap());
    let [b] = input::integers::<usize, 1>(lines.next().unwrap());

    let part1 = iter::successors(Some(0b10), |x| Some((x << 2) | 0b10))
        .find_map(|x| (x >= a * b).then(|| x - a * b))
        .unwrap();

    (part1, 0)
}
