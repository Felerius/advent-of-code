use anyhow::Result;
use itertools::Itertools;
use utils::input;

const OFFSET: u64 = 10_000_000_000_000;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (part1, part2) = input.lines().filter(|line| !line.is_empty()).tuples().fold(
        (0, 0),
        |(part1, part2), (l1, l2, l3)| {
            let [dxa, dya] = input::integers(l1);
            let [dxb, dyb] = input::integers(l2);
            let [x, y] = input::integers(l3);
            let part1 = part1 + solve(x, y, dxa, dya, dxb, dyb).unwrap_or(0);
            let part2 = part2 + solve(x + OFFSET, y + OFFSET, dxa, dya, dxb, dyb).unwrap_or(0);
            (part1, part2)
        },
    );

    Ok((part1, part2))
}

fn solve(x: u64, y: u64, dxa: u64, dya: u64, dxb: u64, dyb: u64) -> Option<u64> {
    let b = (y as f64 - x as f64 * dya as f64 / dxa as f64)
        / (dyb as f64 - dxb as f64 * dya as f64 / dxa as f64);
    let a = (x as f64 - b * dxb as f64) / dxa as f64;

    let a = a.round() as u64;
    let b = b.round() as u64;
    let valid = a * dxa + b * dxb == x && a * dya + b * dyb == y;
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
