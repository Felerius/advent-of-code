use std::iter;

use anyhow::Result;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    // The input code calculates the length of the Collatz sequence for two
    // values. We extract the input values and calculate the lengths manually.
    let mut lines = input.lines().skip(1);
    let part1_input = extract_value(0, lines.by_ref());
    let part2_input = extract_value(1, lines.by_ref());
    Ok((solve(part1_input), solve(part2_input)))
}

fn extract_value<'a>(initial: u32, lines: &mut impl Iterator<Item = &'a str>) -> u32 {
    lines
        .take_while(|line| line.starts_with("inc") || line.starts_with("tpl"))
        .fold(initial, |x, line| {
            if line.starts_with("inc") {
                x + 1
            } else {
                x * 3
            }
        })
}

fn solve(n: u32) -> usize {
    iter::successors(Some(n), |&n| {
        (n > 1).then(|| if n % 2 == 0 { n / 2 } else { 3 * n + 1 })
    })
    .count()
        - 1
}
