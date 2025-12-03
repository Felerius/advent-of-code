mod commands;
mod inputs;
mod style;

use std::process::ExitCode;

use collect::{PuzzleId, Solution};

const ALL_YEARS: &[&[(PuzzleId, Solution)]] = &[
    aoc2015::SOLUTIONS,
    aoc2016::SOLUTIONS,
    aoc2017::SOLUTIONS,
    aoc2024::SOLUTIONS,
    aoc2025::SOLUTIONS,
];

fn all_solutions() -> impl Iterator<Item = (PuzzleId, Solution)> {
    ALL_YEARS.iter().flat_map(|year| year.iter().copied())
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
