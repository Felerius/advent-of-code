use std::ops::Range;

use anyhow::{bail, Result};
use itertools::Itertools;
use utils::input;

const SIZE: usize = 1005;

pub fn run(input: &str) -> Result<(usize, u64)> {
    let instrs: Vec<_> = input.lines().map(parse_instruction).try_collect()?;
    let (xses, x_map) =
        calculate_coordinate_compression(instrs.iter().map(|(_, x_range, _)| x_range.clone()));
    let (yses, y_map) =
        calculate_coordinate_compression(instrs.iter().map(|(_, _, y_range)| y_range.clone()));
    let width = xses.len() - 1;
    let height = yses.len() - 1;

    // Not using `Vec`'s here provides a 3x speedup, probably because of auto vectorization?
    let mut grid1 = Box::new([false; SIZE * SIZE]);
    let mut grid2 = Box::new([0_u16; SIZE * SIZE]);
    for (typ, x_range, y_range) in instrs {
        let x_start = x_map[usize::from(x_range.start)];
        let x_end = x_map[usize::from(x_range.end)];
        let y_start = y_map[usize::from(y_range.start)];
        let y_end = y_map[usize::from(y_range.end)];

        for x in x_start..x_end {
            for y in y_start..y_end {
                let i = x * height + y;
                grid1[i] = match typ {
                    InstructionType::TurnOn => true,
                    InstructionType::TurnOff => false,
                    InstructionType::Toggle => !grid1[i],
                };
                grid2[i] = match typ {
                    InstructionType::TurnOn => grid2[i] + 1,
                    InstructionType::TurnOff => grid2[i].saturating_sub(1),
                    InstructionType::Toggle => grid2[i] + 2,
                };
            }
        }
    }

    let solution =
        (0..width)
            .cartesian_product(0..height)
            .fold((0, 0), |(part1, part2), (x, y)| {
                let block_width = xses[x + 1] - xses[x];
                let block_height = yses[y + 1] - yses[y];
                let block_size = usize::from(block_width) * usize::from(block_height);
                let i = x * height + y;
                (
                    part1 + usize::from(grid1[i]) * block_size,
                    part2 + u64::from(grid2[i]) * block_size as u64,
                )
            });
    Ok(solution)
}

fn calculate_coordinate_compression(
    ranges: impl IntoIterator<Item = Range<u16>>,
) -> (Vec<usize>, [usize; SIZE]) {
    let mut exists = [false; SIZE];
    for i in ranges.into_iter().flat_map(|r| [r.start, r.end]) {
        exists[usize::from(i)] = true;
    }

    let mut vals = Vec::new();
    let mut map = [usize::MAX; SIZE];
    for i in 0..SIZE {
        if exists[i] {
            map[i] = vals.len();
            vals.push(i);
        }
    }

    (vals, map)
}

fn parse_instruction(line: &str) -> Result<(InstructionType, Range<u16>, Range<u16>)> {
    let typ = match &line[..7] {
        "turn on" => InstructionType::TurnOn,
        "turn of" => InstructionType::TurnOff,
        "toggle " => InstructionType::Toggle,
        _ => bail!("unknown instruction type: {line}"),
    };
    let [x1, y1, x2, y2] = input::integers(line);
    Ok((typ, x1..x2 + 1, y1..y2 + 1))
}

enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}
