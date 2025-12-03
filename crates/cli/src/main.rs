#![expect(
    unused_extern_crates,
    reason = "forces linking of the crate for registration macros"
)]
mod commands;
mod inputs;
mod solutions;
mod style;

use std::{
    fmt::{self, Display, Formatter},
    process::ExitCode,
    str::FromStr,
};

use anyhow::Context;
use nutype::nutype;

extern crate aoc2015;
extern crate aoc2016;
extern crate aoc2017;
extern crate aoc2024;
extern crate aoc2025;

fn main() -> ExitCode {
    match commands::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

#[nutype(
    validate(greater_or_equal = 2015),
    derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr
    )
)]
pub(crate) struct Year(u16);

#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 25),
    const_fn,
    derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr
    )
)]
pub(crate) struct Day(u8);

impl Day {
    const TWENTY_FIVE: Self = match Self::try_new(25) {
        Ok(day) => day,
        Err(_) => panic!("invalid hardcoded value"),
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PuzzleId {
    year: Year,
    day: Day,
}

impl Display for PuzzleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.day)
    }
}

impl FromStr for PuzzleId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (year_str, day_str) = s.split_once('-').context("expected <year>-<day>")?;
        let year = year_str.parse().context("invalid year")?;
        let day = day_str.parse().context("invalid day")?;
        Ok(Self { year, day })
    }
}
