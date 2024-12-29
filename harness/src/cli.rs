use std::{
    cmp::Reverse,
    ops::Range,
    process,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use owo_colors::OwoColorize;

use crate::Solution;

pub type SolutionFn = fn(&str) -> Result<Solution>;
pub type SolutionList = [SolutionFn; 25];

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Mode {
    /// Run solutions and print the values for both parts
    Run,

    /// Benchmark solutions and analyze the results
    Bench,
}

#[derive(Parser)]
struct Args {
    #[clap(value_enum)]
    mode: Mode,

    #[clap(value_parser = parse_days_arg, default_value = "1..=25")]
    days: Range<u8>,
}

pub fn run(year: usize, solutions: SolutionList) {
    if let Err(err) = run_fallible(year, solutions) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn run_fallible(year: usize, solutions: SolutionList) -> Result<()> {
    let args = Args::parse();
    let days: Vec<_> = args
        .days
        .map(|day| {
            let day = usize::from(day);
            let input = crate::get_input(year, day)?;
            Ok((day, solutions[day - 1], input))
        })
        .collect::<Result<_>>()?;

    let func = match args.mode {
        Mode::Run => run_solutions,
        Mode::Bench => bench_solutions,
    };
    func(days)
}

fn run_solutions(days: Vec<(usize, SolutionFn, String)>) -> Result<()> {
    let star = "*".fg_rgb::<0xFF, 0xFF, 0x66>();
    for (day, solution, input) in days {
        let output = solution(&input).with_context(|| format!("solution for day {day} failed"))?;
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

fn bench_solutions(days: Vec<(usize, SolutionFn, String)>) -> Result<()> {
    let progress_style = ProgressStyle::with_template("Benchmarking...\n{wide_bar} {pos}/{len}")
        .expect("invalid hardcoded template");
    let progress_bar = ProgressBar::new(days.len() as u64).with_style(progress_style);
    // Immediately print with the bar with zero progress
    progress_bar.tick();

    let mut runtimes: Vec<_> = days
        .into_iter()
        .map(|(day, solution, input)| {
            let runtime = bench_solution(solution, &input)
                .with_context(|| format!("benchmarking day {day} failed"))?;
            Ok((day, runtime))
        })
        .progress_with(progress_bar)
        .collect::<Result<_>>()?;
    runtimes.sort_unstable_by_key(|&(_, runtime)| Reverse(runtime));

    let total_runtime: Duration = runtimes.iter().map(|(_, runtime)| runtime).sum();
    println!("{} {:.2?}", "Total runtime:".blue().bold(), total_runtime);
    println!("{}", "Days (slowest to fastest):".blue().bold());
    for (day, runtime) in runtimes {
        println!("  • {runtime:.2?} (day {day})");
    }

    Ok(())
}

fn bench_solution(solution: SolutionFn, input: &str) -> Result<Duration> {
    let mut times = Vec::new();
    let start = Instant::now();
    for size in (1..).map(|i| 2_usize.pow(i) - 1) {
        if start.elapsed() > Duration::from_secs(1) {
            break;
        }

        while times.len() < size {
            let start_bench = Instant::now();
            solution(input)?;
            let elapsed = start_bench.elapsed();
            times.push(elapsed);
        }
    }

    let median_index = times.len() / 2;
    Ok(*times.select_nth_unstable(median_index).1)
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
