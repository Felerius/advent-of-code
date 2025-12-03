use std::{
    cmp::Reverse,
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;
use jiff::SignedDuration;
use register::SolutionFunction;

use crate::{
    PuzzleId,
    commands::PuzzleArgs,
    inputs,
    style::{error, highlighted, print_runtime_bar, progress_style, spinner},
};

#[derive(clap::Args)]
pub(crate) struct Args {
    #[clap(flatten)]
    puzzles: PuzzleArgs,

    /// Minimum time to run each benchmark for.
    #[clap(short, long, default_value = "1s", value_parser = parse_bench_time)]
    time: Duration,

    /// Render bars to show the relative runtimes of each solution.
    #[clap(long)]
    bar: bool,
}

fn parse_bench_time(s: &str) -> Result<Duration> {
    let duration: SignedDuration = s.parse()?;
    if !duration.is_positive() {
        bail!("must be positive");
    }

    Ok(duration.unsigned_abs())
}

pub(crate) fn run(args: &Args) -> Result<()> {
    let puzzles = args.puzzles.selected_puzzles()?;
    if let &[(puzzle_id, solution)] = puzzles.as_slice() {
        if args.bar {
            eprintln!("{}", error("--bar is only supported for multiple puzzles"));
        }

        let spinner = spinner(format!("Benchmarking {puzzle_id}"), 0);
        let bench_result = benchmark(puzzle_id, solution, args.time)?;
        spinner.finish_and_clear();
        println!("{}: {}", highlighted(puzzle_id), bench_result);
        return Ok(());
    }

    let progress_bar = ProgressBar::new(puzzles.len() as u64).with_style(progress_style());
    progress_bar.tick(); // Immediately print with the bar with zero progress
    let mut benchmarked: Vec<_> = puzzles
        .into_iter()
        .map(|(puzzle_id, solution)| {
            anyhow::Ok((puzzle_id, benchmark(puzzle_id, solution, args.time)?))
        })
        .progress_with(progress_bar)
        .try_collect()?;
    benchmarked.sort_unstable_by_key(|(_, bench_result)| Reverse(bench_result.median));

    let (total_runtime, max_runtime) = benchmarked
        .iter()
        .fold((Duration::ZERO, Duration::ZERO), |(total, max), (_, r)| {
            (total + r.median, max.max(r.median))
        });
    println!("{} {:.2?}", highlighted("Total runtime:"), total_runtime);
    println!("{}:", highlighted("Solutions (slowest to fastest)"));
    for (puzzle_id, bench_result) in benchmarked {
        if args.bar {
            print_runtime_bar(puzzle_id, bench_result.median, max_runtime);
        } else {
            println!("  {}: {}", highlighted(puzzle_id), bench_result);
        }
    }

    Ok(())
}

fn benchmark(
    puzzle_id: PuzzleId,
    solution: SolutionFunction,
    min_time: Duration,
) -> Result<BenchResult> {
    let input = inputs::get(puzzle_id)?;
    let mut runtimes = Vec::new();
    let start = Instant::now();

    // Use 2^i - 1 as a size to have a clear median runtime
    for size in (1..).map(|i| 2_usize.pow(i) - 1) {
        if start.elapsed() > min_time {
            break;
        }

        while runtimes.len() < size {
            let run_start = Instant::now();
            let result = solution(&input);
            let elapsed = run_start.elapsed();
            result.with_context(|| format!("solution for {puzzle_id} failed"))?;
            runtimes.push(elapsed);
        }
    }

    let median_index = runtimes.len() / 2;
    let (low, median, high) = runtimes.select_nth_unstable(median_index);
    let min = *low.iter().min().unwrap_or(median);
    let max = *high.iter().max().unwrap_or(median);
    Ok(BenchResult {
        min,
        median: *median,
        max,
        runs: runtimes.len(),
    })
}

struct BenchResult {
    min: Duration,
    median: Duration,
    max: Duration,
    runs: usize,
}

impl Display for BenchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let details = format!("({:.2?}..{:.2?}, {} runs)", self.min, self.max, self.runs);
        write!(f, "{:.2?} {}", self.median, console::style(details).dim())
    }
}
