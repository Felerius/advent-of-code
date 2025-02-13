use anyhow::{Context, Result};
use utils::input::Input;

pub(crate) fn run(input: &str) -> Result<(u32, u32)> {
    let mut lines = input.lines();
    let [a] = lines
        .nth(19)
        .context("unexpected EOF")?
        .unsigned_integers_n::<u32, 1>()?;
    let [b] = lines
        .next()
        .context("unexpected EOF")?
        .unsigned_integers_n::<u32, 1>()?;
    Ok((5040 + a * b, 479_001_600 + a * b))
}
