use std::{
    cmp::Reverse,
    time::{Duration, Instant},
};

use anyhow::{Result, ensure};
use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;
use jiff::SignedDuration;
use mitsein::vec1::Vec1;
use register::SolutionFunction;

use crate::{
    PuzzleId,
    commands::MultiPuzzleArgs,
    inputs,
    solutions::PuzzleSolutions,
    style::{self, highlighted, print_runtime_bar, progress_style, spinner},
};

#[derive(clap::Args)]
pub(crate) struct Args {
    #[clap(flatten)]
    puzzles: MultiPuzzleArgs,

    /// Minimum time to run each benchmark for.
    #[clap(short, long, default_value = "1s", value_parser = parse_bench_time)]
    time: Duration,

    /// Render bars to show the relative runtimes of each solution.
    #[clap(long)]
    bar: bool,

    /// Benchmark alternative solutions
    #[clap(long, conflicts_with = "bar")]
    alts: bool,
}

fn parse_bench_time(s: &str) -> Result<Duration> {
    let duration: SignedDuration = s.parse()?;
    ensure!(duration.is_positive(), "must be positive");
    Ok(duration.unsigned_abs())
}

pub(crate) fn run(args: &Args) -> Result<()> {
    let puzzles = args.puzzles.evaluate()?;
    if args.bar {
        return benchmark_bars(&puzzles, args.time);
    }

    for &(puzzle_id, solutions) in &puzzles {
        let spinner = spinner(puzzle_id.to_string(), 0);
        let time = benchmark(puzzle_id, solutions.main, args.time)?;
        spinner.finish_and_clear();
        println!("{}: {:>8.2?}", highlighted(puzzle_id), time);

        if args.alts {
            for (name, solution) in &solutions.alts {
                let spinner = style::spinner(name, 2);
                let time = benchmark(puzzle_id, *solution, args.time)?;
                spinner.finish_and_clear();
                println!("  {}: {:>8.2?}", highlighted(name), time);
            }
        }
    }

    Ok(())
}

fn benchmark_bars(puzzles: &[(PuzzleId, &PuzzleSolutions)], min_time: Duration) -> Result<()> {
    let progress_bar = ProgressBar::new(puzzles.len() as u64).with_style(progress_style());
    progress_bar.tick(); // Immediately print with the bar with zero progress
    let mut benchmarked: Vec<_> = puzzles
        .iter()
        .map(|&(puzzle_id, solutions)| {
            anyhow::Ok((puzzle_id, benchmark(puzzle_id, solutions.main, min_time)?))
        })
        .progress_with(progress_bar)
        .try_collect()?;
    benchmarked.sort_unstable_by_key(|&(_, time)| Reverse(time));

    let (total_runtime, max_runtime) = benchmarked.iter().fold(
        (Duration::ZERO, Duration::ZERO),
        |(total, max), &(_, time)| (total + time, max.max(time)),
    );
    println!("{} {:.2?}", highlighted("Total runtime:"), total_runtime);
    println!("{}:", highlighted("Solutions (slowest to fastest)"));
    for (puzzle_id, time) in benchmarked {
        print_runtime_bar(puzzle_id, time, max_runtime);
    }

    Ok(())
}

fn benchmark(
    puzzle_id: PuzzleId,
    solution: SolutionFunction,
    min_time: Duration,
) -> Result<Duration> {
    let input = inputs::get(puzzle_id)?;
    let run = || {
        let start_run = Instant::now();
        solution(&input)?;
        anyhow::Ok(start_run.elapsed())
    };

    let start = Instant::now();
    let mut runs = Vec1::from_one(run()?);
    while start.elapsed() < min_time {
        runs.push(run()?);
    }

    let count = runs.len().get();
    let (_, &mut lower_median, tail) = runs.select_nth_unstable(count / 2);
    let median = if count.is_multiple_of(2) {
        let upper_median = *tail
            .iter()
            .min()
            .expect("we are guaranteed to have at least two elements");
        (lower_median + upper_median) / 2
    } else {
        lower_median
    };

    Ok(median)
}
