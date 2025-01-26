use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run(input: &str) -> Result<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<u32> = line
                .split_ascii_whitespace()
                .map(str::parse)
                .try_collect()?;
            nums.sort_unstable();
            let n = nums.len();

            let part1 = nums[n - 1] - nums[0];
            let part2 = nums
                .iter()
                .tuple_combinations()
                .find_map(|(a, b)| (b % a == 0).then(|| b / a))
                .unwrap_or_default();
            Ok((part1, part2))
        })
        .fold_ok((0, 0), |(part1, part2), (a, b)| (part1 + a, part2 + b))
}
