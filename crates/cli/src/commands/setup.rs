use std::{fs, path::PathBuf};

use anyhow::{Context, Result, ensure};

use crate::{
    Day, PuzzleId,
    commands::{most_recent_puzzle, most_recently_edited_puzzle},
};

const DEFAULT_DAY_RS_CONTENT: &str = "\
use anyhow::Result;
use register::register;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    Ok((0, 0))
}
";

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
pub(crate) struct Args {
    /// Setup the specified puzzle (format: <year>-<day>).
    puzzle: Option<PuzzleId>,

    /// Setup next puzzle after the most recently edited.
    #[clap(long)]
    next: bool,

    /// Setup the most recent puzzle.
    #[clap(long)]
    most_recent: bool,
}

impl Args {
    fn puzzle_id(&self) -> Result<PuzzleId> {
        if self.next {
            let (PuzzleId { year, day }, _) = most_recently_edited_puzzle()?;
            let next_day = Day::try_new(day.into_inner() + 1)
                .with_context(|| format!("no more days in {year}"))?;
            Ok(PuzzleId {
                year,
                day: next_day,
            })
        } else if self.most_recent {
            most_recent_puzzle()
        } else {
            Ok(self
                .puzzle
                .expect("clap should've ensured one argument being present"))
        }
    }
}

pub(crate) fn run(args: &Args) -> Result<()> {
    let id = args.puzzle_id()?;
    let src_dir = PathBuf::from(format!("{}/src", id.year));
    let lib_rs_path = src_dir.join("lib.rs");
    let day_rs_path = src_dir.join(format!("day{:02}.rs", id.day));
    ensure!(!day_rs_path.try_exists()?, "{id} is already setup");
    ensure!(
        lib_rs_path.try_exists()?,
        "year {} is not setup, this must be done manually",
        id.year
    );

    fs::write(day_rs_path, DEFAULT_DAY_RS_CONTENT).context("failed to write solution file")?;

    println!("Set up puzzle {id}");
    Ok(())
}
