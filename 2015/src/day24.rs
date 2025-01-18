use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run(input: &str) -> Result<(u64, u64)> {
    let nums: Vec<_> = input.lines().map(|line| line.parse()).try_collect()?;
    let sum: usize = nums.iter().sum();

    // The inputs in this problem are nice s.t. you can always partition the
    // remaining numbers into 2 or 3 groups. Thus we can restrict our search to
    // the smallest subset with a target sum, tie-breaking by lowest product.
    let set_size_ub = nums.len() / 3;
    let max_target = sum / 3;
    let mut dp = vec![vec![u64::MAX; sum / 3 + 1]; set_size_ub + 1];
    dp[0][0] = 1;
    for i in nums {
        for k in (1..=set_size_ub).rev() {
            for j in (i..=max_target).rev() {
                dp[k][j] = dp[k][j].min(dp[k - 1][j - i].saturating_mul(i as u64));
            }
        }
    }

    let solution_for_target = |target| {
        dp.iter()
            .map(|row| row[target])
            .find(|&x| x != u64::MAX)
            .expect("no solution")
    };
    Ok((solution_for_target(sum / 3), solution_for_target(sum / 4)))
}
