mod bench;
mod run;
mod setup;

use std::{fs, str::FromStr, time::SystemTime};

use anyhow::{Context, Error, Result, bail};
use clap::Parser;
use indicatif::ProgressBar;
use jiff::{
    Zoned,
    tz::{Offset, TimeZone},
};
use mitsein::{iter1::IteratorExt, vec1::Vec1};

use crate::{
    Day, PuzzleId, Year,
    solutions::{PuzzleSolutions, Solutions},
    style::progress_style,
};

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

#[derive(clap::Args)]
#[group(required = false, multiple = false)]
struct MultiPuzzleArgs {
    /// Select puzzles to run by year and/or day.
    ///
    /// Supports the following formats: <year>-<day>, <year>, and <day> (in the
    /// most recent year).
    puzzles: Option<MultiPuzzleSelector>,

    /// Run the puzzle whose solution file was most recently edited (this is the
    /// default).
    #[clap(long)]
    most_recently_edited: bool,

    /// Run all solved puzzles.
    #[clap(long)]
    all: bool,

    /// Run the most recently released puzzle.
    #[clap(long)]
    most_recent: bool,
}

impl MultiPuzzleArgs {
    fn evaluate(&self) -> Result<Vec1<(PuzzleId, &'static PuzzleSolutions)>> {
        let (maybe_year, maybe_day) = if let Some(selector) = self.puzzles {
            match selector {
                MultiPuzzleSelector::DayInMostRecentYear(day) => {
                    let year = most_recent_puzzle()?.year;
                    (Some(year), Some(day))
                }
                MultiPuzzleSelector::Year(year) => (Some(year), None),
                MultiPuzzleSelector::Single(id) => (Some(id.year), Some(id.day)),
            }
        } else if self.all {
            (None, None)
        } else if self.most_recent {
            let id = most_recent_puzzle()?;
            (Some(id.year), Some(id.day))
        } else {
            return most_recently_edited_puzzle().map(Vec1::from_one);
        };

        let mut selected: Vec1<_> = Solutions::get()?
            .by_id
            .iter()
            .map(|(id, solutions)| (*id, solutions))
            .filter(|(id, _)| {
                maybe_year.is_none_or(|year| id.year == year)
                    && maybe_day.is_none_or(|day| id.day == day)
            })
            .try_collect1()
            .ok()
            .context("no puzzles found for selection")?;
        selected.sort_unstable_by_key(|(id, _)| *id);
        Ok(selected)
    }
}

#[derive(Debug, Clone, Copy)]
enum MultiPuzzleSelector {
    DayInMostRecentYear(Day),
    Year(Year),
    Single(PuzzleId),
}

impl FromStr for MultiPuzzleSelector {
    type Err = Error;

    #[expect(
        clippy::same_functions_in_if_condition,
        reason = "we're parsing different types"
    )]
    fn from_str(s: &str) -> Result<Self> {
        if let Ok(id) = s.parse() {
            Ok(Self::Single(id))
        } else if let Ok(year) = s.parse() {
            Ok(Self::Year(year))
        } else if let Ok(day) = s.parse() {
            Ok(Self::DayInMostRecentYear(day))
        } else {
            bail!("invalid puzzle selector: {s:?}")
        }
    }
}

fn most_recent_puzzle() -> Result<PuzzleId> {
    let est = TimeZone::fixed(Offset::constant(-5));
    let date_est = Zoned::now().with_time_zone(est).date();
    let year = u16::try_from(date_est.year()).context("current year is negative!?")?;
    let (year, day) = if date_est.month() == 12 {
        #[expect(
            clippy::cast_sign_loss,
            reason = "day() is guaranteed to be in [1, 31]"
        )]
        let day = date_est.day() as u8;
        let day = Day::try_new(day.min(25)).expect("check should ensure validity");
        (year, day)
    } else {
        (year - 1, Day::TWENTY_FIVE)
    };

    let year = Year::try_new(year).context("before the start of AoC")?;
    Ok(PuzzleId { year, day })
}

fn most_recently_edited_puzzle() -> Result<(PuzzleId, &'static PuzzleSolutions)> {
    let solutions = Solutions::get()?;
    let (_, id) = solutions
        .by_file
        .iter()
        .map(|(&file, &id)| {
            let modified_time = file_modified_time(file)
                .with_context(|| format!("failed to determine modification time of {file:?}"))?;
            anyhow::Ok((modified_time, id))
        })
        .reduce(|sol1, sol2| {
            let (t1, id1) = sol1?;
            let (t2, id2) = sol2?;
            if t1 > t2 {
                Ok((t1, id1))
            } else {
                Ok((t2, id2))
            }
        })
        .context("no puzzles found")??;
    Ok((id, &solutions.by_id[&id]))
}

fn file_modified_time(path: &str) -> Result<SystemTime> {
    Ok(fs::metadata(path)?.modified()?)
}

fn init_progress_bar(puzzles: &[(PuzzleId, &PuzzleSolutions)], alts: bool) -> ProgressBar {
    let total = if alts {
        puzzles
            .iter()
            .map(|(_, solutions)| 1 + solutions.alts.len())
            .sum()
    } else {
        puzzles.len()
    };
    let progress_bar = ProgressBar::new(total as u64).with_style(progress_style());

    // Immediately print with the bar with zero progress
    progress_bar.tick();

    progress_bar
}
