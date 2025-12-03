use std::{array, cmp::Reverse};

use anyhow::Result;
use register::register;
use utils::input::Input;

mod vm;

const N: usize = 127;

#[register]
fn run(input: &str) -> Result<(u16, usize)> {
    // The program with p = 0 generates 127 pseudo-random numbers based on an
    // initial seed that differs from input to input. The two programs then
    // implement bubble sort, each doing one iteration of adjacent swaps per
    // round, until the list is sorted. The answer for part 2 thus depends on
    // the number of bubble sort iterations needed to sort the numbers in
    // descending order.
    let [mut seed]: [u64; 1] = input.lines().nth(9).unwrap().unsigned_integers_n()?;
    let nums: [_; N] = array::from_fn(|_| {
        seed = (seed * 8505 * 129_749 + 12345) % 0x7FFF_FFFF;
        (seed % 10_000) as u16
    });
    let part1 = nums[N - 1];

    // To determine the number of iterations, we look at how far numbers need to
    // move left. While a number can move arbitrarily far to the right in one
    // iteration, it can only move one position to the left. Thus, we can find
    // the farthest a number needs to move to the left and finally add one
    // additional iteration which would detect that the array is sorted.
    let mut indices: [_; N] = array::from_fn(|i| i);
    indices.sort_by_key(|&i| Reverse(nums[i]));
    let iters = indices
        .into_iter()
        .enumerate()
        .map(|(sorted_idx, orig_idx)| orig_idx.saturating_sub(sorted_idx))
        .max()
        .unwrap_or_default();
    let part2 = N * (iters + 1).div_ceil(2);

    Ok((part1, part2))
}
