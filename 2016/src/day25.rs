use std::iter;

use anyhow::{Context, Result};
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(usize, u8)> {
    let mut lines = input.lines();
    let [a] = lines
        .nth(1)
        .context("unexpected EOF")?
        .unsigned_integers_n::<usize, 1>()?;
    let [b] = lines
        .next()
        .context("unexpected EOF")?
        .unsigned_integers_n::<usize, 1>()?;

    let part1 = iter::successors(Some(0b10), |x| Some((x << 2) | 0b10))
        .find_map(|x| (x >= a * b).then(|| x - a * b))
        .unwrap();

    Ok((part1, 0))
}
