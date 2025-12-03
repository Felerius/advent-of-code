#![expect(
    unused_extern_crates,
    reason = "forces linking of the crate for registration macros"
)]
mod commands;
mod inputs;
mod style;

use std::{
    fmt::{self, Display, Formatter},
    process::ExitCode,
};

use register::{RegisteredFunction, SolutionFunction};

extern crate aoc2015;
extern crate aoc2016;
extern crate aoc2017;
extern crate aoc2024;
extern crate aoc2025;

fn all_solutions() -> impl Iterator<Item = (PuzzleId, SolutionFunction)> {
    RegisteredFunction::all().iter().map(|reg_fn| {
        // TODO: error handling
        let (krate, module) = reg_fn.module_path.split_once("::").unwrap();
        let year = krate.strip_prefix("aoc").unwrap().parse().unwrap();
        let day = module.strip_prefix("day").unwrap().parse().unwrap();
        (PuzzleId { year, day }, reg_fn.func)
    })
}

fn main() -> ExitCode {
    match commands::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PuzzleId {
    pub year: u16,
    pub day: u8,
}

impl Display for PuzzleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.day)
    }
}
