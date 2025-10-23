use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use collect::PuzzleId;
use regex_lite::RegexBuilder;

use crate::commands::{most_recent_puzzle, most_recently_edited_puzzle};

const DEFAULT_DAY_RS_CONTENT: &str = "\
use anyhow::Result;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    Ok((0, 0))
}
";

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
pub(crate) struct Args {
    /// Setup the specified puzzle (format: <year>-<day>).
    #[clap(value_parser = parse_puzzle_id)]
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
            let last = most_recently_edited_puzzle()?;
            ensure!(last.day < 25, "no more puzzles in {}", last.year);
            Ok(PuzzleId {
                year: last.year,
                day: last.day + 1,
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

fn parse_puzzle_id(s: &str) -> Result<PuzzleId> {
    let (year, day) = s.split_once('-').context("invalid <year>-<day>")?;
    let year = year.parse().context("invalid year")?;
    let day = day.parse().context("invalid day")?;
    Ok(PuzzleId { year, day })
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

    adjust_lib_rs(&lib_rs_path, id)?;
    fs::write(day_rs_path, DEFAULT_DAY_RS_CONTENT).context("failed to write solution file")?;

    Ok(())
}

fn adjust_lib_rs(lib_rs_path: &Path, id: PuzzleId) -> Result<()> {
    let mut contents = fs::read_to_string(lib_rs_path).context("failed to read lib.rs")?;
    let regex = RegexBuilder::new(r"^collect::collect!\(\d+; ([\d, ]+)\);$")
        .multi_line(true)
        .build()
        .expect("error in hardcoded regex");
    let captures = regex
        .captures(&contents)
        .context("could not find collect::collect! call in lib.rs")?;

    let mut days: Vec<_> = captures[1]
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    days.push(format!("{:02}", id.day));
    days.sort_unstable();
    let new_days_str = days.join(", ");

    let range = captures.get(1).unwrap().range();
    contents.replace_range(range, &new_days_str);
    fs::write(lib_rs_path, contents).context("failed to write lib.rs")?;
    Ok(())
}
