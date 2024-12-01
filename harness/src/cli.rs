use std::{ops::Range, process};

use anyhow::{Context, Result};
use clap::Parser;
use owo_colors::OwoColorize;

use crate::Solution;

pub type SolutionList = [fn(&str) -> Result<Solution>; 25];

#[derive(Parser)]
struct Args {
    #[clap(value_parser = parse_days_arg, default_value = "1..=25")]
    days: Range<u8>,
}

fn run_fallible(year: usize, solutions: SolutionList) -> Result<()> {
    let args = Args::parse();
    let star = "*".fg_rgb::<0xFF, 0xFF, 0x66>();
    for day in args.days.map(usize::from) {
        let input = crate::get_input(year, day)?;
        let output =
            solutions[day - 1](&input).with_context(|| format!("solution for day {day} failed"))?;
        if let Solution(Some((part1, part2))) = output {
            println!("{}", format_args!("Day {day}").blue().bold());
            println!("  {star} Part 1: {part1}");
            if day != 25 {
                println!("  {star} Part 2: {part2}");
            }
        }
    }

    Ok(())
}

pub fn run(year: usize, solutions: SolutionList) {
    if let Err(err) = run_fallible(year, solutions) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn parse_days_arg(s: &str) -> Result<Range<u8>> {
    if let Some((lower, upper)) = s.split_once("..=") {
        let lower = parse_bound_or_empty(lower, 1)?;
        let upper = parse_bound_or_empty(upper, 25)?;
        Ok(lower..upper + 1)
    } else if let Some((lower, upper)) = s.split_once("..") {
        let lower = parse_bound_or_empty(lower, 1)?;
        let upper = parse_bound_or_empty(upper, 26)?;
        Ok(lower..upper)
    } else {
        let day = s.parse().context("failed to parse day number")?;
        Ok(day..day + 1)
    }
}

fn parse_bound_or_empty(s: &str, default: u8) -> Result<u8> {
    if s.is_empty() {
        Ok(default)
    } else {
        s.parse().context("failed to parse range bound")
    }
}

#[macro_export]
macro_rules! cli {
    ($year:literal) => {
        $crate::cli!(@private $year, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
    };
    (@private $year:literal, $($day:literal),*) => {
        fn main() {
            $crate::__macro_support::concat_idents!(lib_crate_name = aoc, $year {
                use lib_crate_name as lib_crate;
            });
            let solutions: $crate::cli::SolutionList = [
                $(
                    |input| {
                        let solution = $crate::__macro_support::concat_idents!(day_mod = day, $day {
                            lib_crate::day_mod::run(input)
                        });
                        $crate::IntoResultSolution::into(solution)
                    },
                )*
            ];
            $crate::cli::run($year, solutions);
        }
    };
}
