use anyhow::Result;
use itertools::Itertools;
use utils::input::Input;

const OFFSET: i64 = 10_000_000_000_000;

pub(crate) fn run(input: &str) -> Result<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples()
        .try_fold((0, 0), |(part1, part2), (l1, l2, l3)| {
            let [x1, x2] = l1.signed_integers_n()?;
            let [y1, y2] = l2.signed_integers_n()?;
            let [z1, z2] = l3.signed_integers_n()?;
            let part1 = part1 + solve(z1, z2, x1, y1, x2, y2).unwrap_or(0);
            let part2 = part2 + solve(z1 + OFFSET, z2 + OFFSET, x1, y1, x2, y2).unwrap_or(0);
            Ok((part1, part2))
        })
}

fn solve(z1: i64, z2: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> Option<i64> {
    let b = (z1 * x2 - z2 * x1) / (y1 * x2 - y2 * x1);
    let a = (z1 - b * y1) / x1;
    let valid = a * x1 + b * y1 == z1 && a * x2 + b * y2 == z2;
    valid.then_some(3 * a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).unwrap().0, 480);
    }
}
