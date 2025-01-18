use anyhow::{Context, Result};
use itertools::Itertools;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let vals: Vec<usize> = input.lines().map(|l| l.parse()).try_collect()?;
    let n = vals.len();
    let mut dp = vec![[0; 151]; n + 1];
    dp[0][0] = 1;
    for (i, vi) in vals.into_iter().enumerate() {
        for j in (1..i + 2).rev() {
            for k in vi..151 {
                dp[j][k] += dp[j - 1][k - vi];
            }
        }
    }

    let part1 = (1..=n).map(|i| dp[i][150]).sum::<usize>();
    let part2 = (1..=n)
        .map(|i| dp[i][150])
        .find(|&x| x > 0)
        .context("no solution for 150 liters")?;

    Ok((part1, part2))
}
