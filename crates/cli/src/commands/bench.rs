use std::{
    cmp::Reverse,
    time::{Duration, Instant},
};

use anyhow::{Result, ensure};
use indicatif::ProgressIterator;
use itertools::Itertools;
use jiff::SignedDuration;
use mitsein::vec1::Vec1;
use register::SolutionFunction;

use crate::{
    PuzzleId,
    commands::{MultiPuzzleArgs, init_progress_bar},
    inputs,
    solutions::PuzzleSolutions,
    style::{AOC_STAR, DIM, HIGHLIGHT, print_runtime_bar},
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

pub(crate) fn run(args: &Args) -> Result<()> {
    let puzzles = args.puzzles.evaluate()?;
    if args.bar {
        run_bars(&puzzles, args.time)
    } else {
        run_normal(&puzzles, args)
    }
}

fn run_normal(puzzles: &[(PuzzleId, &PuzzleSolutions)], args: &Args) -> Result<()> {
    let progress_bar = init_progress_bar(puzzles, args.alts);
    for &(puzzle_id, solutions) in puzzles {
        let main_time = benchmark(puzzle_id, solutions.main, args.time)?;
        let message = format!(
            "{AOC_STAR} {} {main_time:>8.2?}",
            HIGHLIGHT.apply_to(puzzle_id),
        );
        progress_bar.inc(1);
        progress_bar.println(message);

        if args.alts {
            for (alt_name, alt_solution) in &solutions.alts {
                let alt_time = benchmark(puzzle_id, *alt_solution, args.time)?;
                let factor = alt_time.as_secs_f64() / main_time.as_secs_f64();
                let digits = if factor >= 100.0 {
                    0
                } else if factor >= 10.0 {
                    1
                } else {
                    2
                };
                let paren_info = format!("({alt_name}, {factor:.digits$}x slower)");
                let message = format!("{alt_time:>18.2?} {}", DIM.apply_to(paren_info));
                progress_bar.inc(1);
                progress_bar.println(message);
            }
        }
    }

    Ok(())
}

fn run_bars(puzzles: &[(PuzzleId, &PuzzleSolutions)], min_time: Duration) -> Result<()> {
    let progress_bar = init_progress_bar(puzzles, false);
    let mut benchmarked: Vec<_> = puzzles
        .iter()
        .map(|&(puzzle_id, solutions)| {
            anyhow::Ok((puzzle_id, benchmark(puzzle_id, solutions.main, min_time)?))
        })
        .progress_with(progress_bar)
        .try_collect()?;
    benchmarked.sort_unstable_by_key(|&(_, time)| Reverse(time));

    let (total, max) = benchmarked.iter().fold(
        (Duration::ZERO, Duration::ZERO),
        |(total, max), &(_, time)| (total + time, max.max(time)),
    );
    println!("{} {total:.2?}", HIGHLIGHT.apply_to("Total runtime:"));
    println!("{}", HIGHLIGHT.apply_to("Solutions (slowest to fastest):"));
    for (puzzle_id, time) in benchmarked {
        print_runtime_bar(puzzle_id, time, max);
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

fn parse_bench_time(s: &str) -> Result<Duration> {
    let duration: SignedDuration = s.parse()?;
    ensure!(duration.is_positive(), "must be positive");
    Ok(duration.unsigned_abs())
}
