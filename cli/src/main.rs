mod commands;
mod inputs;
mod style;

use std::process::ExitCode;

use collect::{PuzzleId, Solution};

const ALL_YEARS: &[&[(PuzzleId, Solution)]] =
    &[aoc2015::SOLUTIONS, aoc2016::SOLUTIONS, aoc2024::SOLUTIONS];

fn main() -> ExitCode {
    match commands::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}
