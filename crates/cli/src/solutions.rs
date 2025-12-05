use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use register::{RegisteredFunction, SolutionFunction};

use crate::PuzzleId;

#[derive(Debug, Clone)]
pub(crate) struct Solutions {
    pub(crate) by_id: HashMap<PuzzleId, PuzzleSolutions>,
    pub(crate) by_file: HashMap<&'static str, PuzzleId>,
}

impl Solutions {
    pub(crate) fn get() -> Result<&'static Self> {
        static INSTANCE: OnceCell<Solutions> = OnceCell::new();
        INSTANCE.get_or_try_init(Self::collect)
    }

    fn collect() -> Result<Self> {
        let mut by_id: Vec<_> = RegisteredFunction::all()
            .iter()
            .map(|&reg_fn| {
                let (id, tail_modules) = parse_module_path(reg_fn.module_path)
                    .context("expected function to be in a module like `aoc<year>::day<day>`")?;
                anyhow::Ok((id, tail_modules, reg_fn))
            })
            .try_collect()?;
        by_id.sort_unstable_by_key(|(id, _, _)| *id);

        let mut by_file = HashMap::new();
        let by_id = by_id
            .chunk_by(|(id1, _, _), (id2, _, _)| id1 == id2)
            .map(|solutions| PuzzleSolutions::collect(solutions, &mut by_file))
            .try_collect()?;

        Ok(Self { by_id, by_file })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PuzzleSolutions {
    pub(crate) main: SolutionFunction,
    pub(crate) alts: Vec<(String, SolutionFunction)>,
}

impl PuzzleSolutions {
    fn collect(
        solutions: &[(PuzzleId, &'static str, RegisteredFunction)],
        by_file: &mut HashMap<&'static str, PuzzleId>,
    ) -> Result<(PuzzleId, Self)> {
        let (id, _, _) = solutions[0];
        let files = solutions.iter().map(|(_, _, reg_fn)| reg_fn.file);
        for file in files {
            if let Some(other_id) = by_file.insert(file, id).filter(|&i| i != id) {
                bail!("found solutions for both {id} and {other_id} in the same file {file}");
            }
        }

        let mut main = None;
        let alts = solutions
            .iter()
            .filter_map(|&(_, tail_modules, reg_fn)| {
                if tail_modules.is_empty() && reg_fn.name == "run" {
                    main = Some(reg_fn.func);
                    None
                } else {
                    let alt_name = reg_fn.name.replace('_', " ");
                    let alt_name = if tail_modules.is_empty() {
                        alt_name
                    } else {
                        let module = tail_modules.replace("::", "/").replace('_', " ");
                        format!("{module} {alt_name}")
                    };
                    Some((alt_name, reg_fn.func))
                }
            })
            .collect();

        let main = main.with_context(|| format!("no main solution found for {id}"))?;
        Ok((id, Self { main, alts }))
    }
}

fn parse_module_path(module_path: &str) -> Option<(PuzzleId, &str)> {
    let (krate, module) = module_path.split_once("::")?;
    let (top_level_module, tail_modules) = module.split_once("::").unwrap_or((module, ""));
    let year = krate.strip_prefix("aoc")?.parse().ok()?;
    let day = top_level_module.strip_prefix("day")?.parse().ok()?;
    Some((PuzzleId { year, day }, tail_modules))
}
