mod bench;
mod run;

use std::str::FromStr;

use anyhow::{bail, format_err, Context, Error, Result};
use clap::Parser;
use collect::{PuzzleId, Solution};
use jiff::{
    tz::{Offset, TimeZone},
    Zoned,
};

use crate::all_solutions;

#[derive(Parser)]
enum Args {
    /// Run one or multiple puzzle solutions.
    Run(run::Args),

    /// Benchmark one or multiple puzzle solutions.
    Bench(bench::Args),
}

pub(crate) fn run() -> Result<()> {
    let opts = Args::parse();
    match opts {
        Args::Run(args) => run::run(&args),
        Args::Bench(args) => bench::run(&args),
    }
}

#[derive(Debug, Clone, Copy)]
enum PuzzleSelection {
    All,
    MostRecent,
    DayInMostRecentYear(u8),
    Year(u16),
    Single(PuzzleId),
}

impl FromStr for PuzzleSelection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.eq_ignore_ascii_case("all") {
            Ok(Self::All)
        } else if let Some((year, day)) = s.split_once('-') {
            let year = year.parse().context("invalid <year> in <year>-<day>")?;
            let day = day.parse().context("invalid <day> in <year>-<day>")?;
            Ok(Self::Single(PuzzleId { year, day }))
        } else if let Ok(num) = s.parse() {
            if let Some(day) = u8::try_from(num).ok().filter(|day| (1..=25).contains(day)) {
                Ok(Self::DayInMostRecentYear(day))
            } else {
                Ok(Self::Year(num))
            }
        } else {
            bail!("invalid puzzle selection")
        }
    }
}

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
struct PuzzleArgs {
    /// Which puzzles to run.
    ///
    /// Supports the following values:
    ///
    /// - no value: run the most recent puzzle (unless `--most-recently-edited` is used)
    /// - `all`: run all solved puzzles
    /// - `<day>`: run <day> in the most recent year
    /// - `<year>`: run all puzzles in <year>
    /// - `<year>-<day>`: run <day> in <year>
    #[clap(verbatim_doc_comment)]
    puzzles: Option<PuzzleSelection>,

    /// Run the puzzle whose solution file was most recently edited.
    #[clap(long)]
    most_recently_edited: bool,
}

impl PuzzleArgs {
    fn most_recent_puzzle() -> Result<PuzzleId> {
        let est = TimeZone::fixed(Offset::constant(-5));
        let date_est = Zoned::now().with_time_zone(est).date();
        let year = u16::try_from(date_est.year())
            .context("current year is negative, AoC didn't exist yet")?;
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

    fn selected_filters(&self) -> (Option<u16>, Option<u8>) {
        if self.most_recently_edited {
            let most_recent = Self::most_recently_edited_puzzle().unwrap();
            return (Some(most_recent.year), Some(most_recent.day));
        }

        match self.puzzles.unwrap_or(PuzzleSelection::MostRecent) {
            PuzzleSelection::All => (None, None),
            PuzzleSelection::MostRecent => {
                let most_recent = Self::most_recent_puzzle().unwrap();
                (Some(most_recent.year), Some(most_recent.day))
            }
            PuzzleSelection::DayInMostRecentYear(day) => {
                let most_recent = Self::most_recent_puzzle().unwrap();
                (Some(most_recent.year), Some(day))
            }
            PuzzleSelection::Year(year) => (Some(year), None),
            PuzzleSelection::Single(puzzle_id) => (Some(puzzle_id.year), Some(puzzle_id.day)),
        }
    }

    fn selected_puzzles(&self) -> Result<Vec<(PuzzleId, Solution)>> {
        let (chosen_year, chosen_day) = self.selected_filters();
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
