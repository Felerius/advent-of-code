mod bench;
mod run;
mod setup;

use std::str::FromStr;

use anyhow::{Context, Error, Result, bail, format_err};
use clap::Parser;
use jiff::{
    Zoned,
    tz::{Offset, TimeZone},
};
use register::SolutionFunction;

use crate::{PuzzleId, all_solutions};

#[derive(Parser)]
enum Args {
    /// Run one or multiple puzzle solutions.
    Run(run::Args),

    /// Benchmark one or multiple puzzle solutions.
    Bench(bench::Args),

    /// Setup a new, empty puzzle solution.
    Setup(setup::Args),
}

pub(crate) fn run() -> Result<()> {
    let opts = Args::parse();
    match opts {
        Args::Run(args) => run::run(&args),
        Args::Bench(args) => bench::run(&args),
        Args::Setup(args) => setup::run(&args),
    }
}

#[derive(Debug, Clone, Copy)]
enum PuzzleSelection {
    DayInMostRecentYear(u8),
    Year(u16),
    Single(PuzzleId),
}

impl FromStr for PuzzleSelection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((year, day)) = s.split_once('-') {
            let year = year.parse().context("invalid <year> in <year>-<day>")?;
            let day = day.parse().context("invalid <day> in <year>-<day>")?;
            return Ok(Self::Single(PuzzleId { year, day }));
        }

        if let Ok(num) = s.parse() {
            if let Some(day) = u8::try_from(num).ok().filter(|day| (1..=25).contains(day)) {
                return Ok(Self::DayInMostRecentYear(day));
            } else if num >= 2015 {
                return Ok(Self::Year(num));
            }
        }

        bail!("invalid puzzle selection")
    }
}

#[derive(clap::Args)]
#[group(required = false, multiple = false)]
struct PuzzleArgs {
    /// Select puzzles to run by year and/or day.
    ///
    /// Supports the following formats:
    ///
    /// - `<day>`: run <day> in the most recent year
    /// - `<year>`: run all puzzles in <year>
    /// - `<year>-<day>`: run <day> in <year>
    #[clap(verbatim_doc_comment)]
    puzzles: Option<PuzzleSelection>,

    /// Run the puzzle whose solution file was most recently edited (this is the default).
    #[clap(long)]
    most_recently_edited: bool,

    /// Run all solved puzzles.
    #[clap(long)]
    all: bool,

    /// Run the most recently released puzzle.
    #[clap(long)]
    most_recent: bool,
}

impl PuzzleArgs {
    fn derive_filters(&self) -> Result<(Option<u16>, Option<u8>)> {
        let selection = if let Some(selection) = self.puzzles {
            Some(selection)
        } else if self.all {
            None
        } else if self.most_recent {
            Some(PuzzleSelection::Single(most_recent_puzzle()?))
        } else {
            Some(PuzzleSelection::Single(most_recently_edited_puzzle()?))
        };

        let (chosen_year, chosen_day) = match selection {
            Some(PuzzleSelection::DayInMostRecentYear(day)) => {
                let year = most_recent_puzzle()?.year;
                (Some(year), Some(day))
            }
            Some(PuzzleSelection::Year(year)) => (Some(year), None),
            Some(PuzzleSelection::Single(id)) => (Some(id.year), Some(id.day)),
            None => (None, None),
        };

        Ok((chosen_year, chosen_day))
    }

    fn selected_puzzles(&self) -> Result<Vec<(PuzzleId, SolutionFunction)>> {
        let (chosen_year, chosen_day) = self.derive_filters()?;
        let selected: Vec<_> = all_solutions()
            .filter(|(id, _)| {
                chosen_year.is_none_or(|year| id.year == year)
                    && chosen_day.is_none_or(|day| id.day == day)
            })
            .collect();

        if selected.is_empty() {
            bail!("no puzzles found for selection")
        }

        Ok(selected)
    }
}

fn most_recent_puzzle() -> Result<PuzzleId> {
    let est = TimeZone::fixed(Offset::constant(-5));
    let date_est = Zoned::now().with_time_zone(est).date();
    let year = u16::try_from(date_est.year())
        .context("current year is negative, AoC doesn't exist yet")?;
    let (year, day) = if date_est.month() == 12 {
        #[allow(
            clippy::cast_sign_loss,
            reason = "day() is guaranteed to be in [1, 31]"
        )]
        (year, (date_est.day() as u8).min(25))
    } else {
        (year - 1, 25)
    };
    Ok(PuzzleId { year, day })
}

fn most_recently_edited_puzzle() -> Result<PuzzleId> {
    all_solutions()
        .filter_map(|(id, _)| {
            let source_file = format!("{}/src/day{:02}.rs", id.year, id.day);
            let metadata = std::fs::metadata(source_file).ok()?;
            let modified = metadata.modified().ok()?;
            Some((id, modified))
        })
        .max_by_key(|&(_, modified)| modified)
        .map(|(id, _)| id)
        .ok_or_else(|| format_err!("No puzzles found. Wrong directory?"))
}
