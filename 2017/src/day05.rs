use std::iter;

use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let jumps: Vec<_> = input.lines().map(str::parse).try_collect()?;
    let part1 = simulate(jumps.clone(), |i| i + 1);
    let part2 = simulate(jumps, |i| if i >= 3 { i - 1 } else { i + 1 });
    Ok((part1, part2))
}

fn simulate(mut jumps: Vec<isize>, mut change_jump: impl FnMut(isize) -> isize) -> usize {
    iter::successors(Some(0), |&i| {
        let j = usize::try_from(i).ok()?;
        let jmp = jumps.get_mut(j)?;
        let i = i + *jmp;
        *jmp = change_jump(*jmp);
        Some(i)
    })
    .skip(1)
    .count()
}
