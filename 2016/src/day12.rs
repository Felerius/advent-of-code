use anyhow::{Context, Result};
use itertools::Itertools;
use utils::input;

use crate::assembunny::{Register, VirtualMachine};

pub(crate) fn run(input: &str) -> Result<(u32, u32)> {
    let mut lines = input.lines();
    let [index1] = input::integers(lines.nth(2).context("unexpected eof")?);
    let [off] = input::integers(lines.nth(2).context("unexpected eof")?);
    let [factor1]: [u32; 1] = input::integers(lines.nth(10).context("unexpected eof")?);
    let [factor2]: [u32; 1] = input::integers(lines.next().context("unexpected eof")?);

    let mut a: u32 = 1;
    let mut b = 2;
    for _ in 0..index1 {
        (a, b) = (b, a + b);
    }
    let part1 = a + factor1 * factor2;

    for _ in 0..off {
        (a, b) = (b, a + b);
    }

    Ok((part1, a + factor1 * factor2))
}

#[allow(dead_code)]
fn run_interpreted(input: &str) -> Result<(u32, u32)> {
    let program: Vec<_> = input.lines().map(str::parse).try_collect()?;

    let mut vm = VirtualMachine::<4>::new();
    let part1 = vm.execute(&program)?;

    vm.reset();
    vm.set(Register::C, 1);
    let part2 = vm.execute(&program)?;

    Ok((part1, part2))
}
