use anyhow::Result;
use itertools::Itertools;

use crate::knot_hash;

pub(crate) fn run(input: &str) -> Result<(u16, String)> {
    let part1_lengths: Vec<_> = input.split(',').map(str::parse).try_collect()?;
    let part1_hash = knot_hash::hash_lengths(part1_lengths);
    let part1 = u16::from(part1_hash[0]) * u16::from(part1_hash[1]);
    let part2 = knot_hash::hash_str(input.trim());
    let part2 = format!("{part2:032x}");

    Ok((part1, part2))
}
