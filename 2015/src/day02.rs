use anyhow::Result;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(u64, u64)> {
    input.lines().try_fold((0, 0), |(part1, part2), line| {
        let mut nums: [u64; 3] = line.unsigned_integers_n()?;
        nums.sort_unstable();
        let [a, b, c] = nums;

        Ok((
            part1 + 3 * a * b + 2 * (a * c + b * c),
            part2 + 2 * (a + b) + a * b * c,
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let inputs = [("2x3x4", 58), ("1x1x10", 43)];
        for (input, expected) in inputs {
            assert_eq!(run(input).unwrap().0, expected, "failed for {input:?}");
        }
    }

    #[test]
    fn part2() {
        let inputs = [("2x3x4", 34), ("1x1x10", 14)];
        for (input, expected) in inputs {
            assert_eq!(run(input).unwrap().1, expected, "failed for {input:?}");
        }
    }
}
