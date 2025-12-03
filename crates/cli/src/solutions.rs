use anyhow::{Context, Result};
use register::{RegisteredFunction, SolutionFunction};

use crate::PuzzleId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DaySolution {
    pub func: SolutionFunction,
    pub file: &'static str,
}

pub(crate) fn collect() -> Result<Vec<(PuzzleId, DaySolution)>> {
    RegisteredFunction::all()
        .iter()
        .map(|reg_fn| {
            let id = parse_module_path(reg_fn.module_path)
                .context("expexted function to be in a module like `aoc<year>::day<day>`")?;
            let solution = DaySolution {
                func: reg_fn.func,
                file: reg_fn.file,
            };
            Ok((id, solution))
        })
        .collect()
}

fn parse_module_path(module_path: &str) -> Option<PuzzleId> {
    let (krate, module) = module_path.split_once("::")?;
    let year = krate.strip_prefix("aoc")?.parse().ok()?;
    let day = module.strip_prefix("day")?.parse().ok()?;
    Some(PuzzleId { year, day })
}
