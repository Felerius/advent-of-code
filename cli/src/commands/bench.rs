use std::{
    cmp::Reverse,
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

use anyhow::{bail, Context, Result};
use collect::{PuzzleId, Solution};
use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;
use jiff::SignedDuration;

use crate::{
    commands::PuzzleArgs,
    inputs,
    style::{highlighted, spinner},
};

#[derive(clap::Args)]
pub(crate) struct Args {
    #[clap(flatten)]
    puzzles: PuzzleArgs,

    /// Minimum time to run each benchmark for.
    #[clap(short, long, default_value = "1s", value_parser = parse_bench_time)]
    time: Duration,
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
    if puzzles.len() == 1 {
        let spinner = spinner("benchmarking", 0);
        let (puzzle_id, solution) = puzzles[0];
        let bench_result = benchmark(puzzle_id, solution, args.time)?;
        spinner.finish_and_clear();
        println!("{}: {}", highlighted(puzzle_id), bench_result);
        return Ok(());
    }

    let progress_bar = ProgressBar::new(puzzles.len() as u64);
    // Immediately print with the bar with zero progress
    progress_bar.tick();

    let mut benchmarked: Vec<_> = puzzles
        .into_iter()
        .map(|(puzzle_id, solution)| {
            anyhow::Ok((puzzle_id, benchmark(puzzle_id, solution, args.time)?))
        })
        .progress_with(progress_bar)
        .try_collect()?;
    benchmarked.sort_unstable_by_key(|(_, bench_result)| Reverse(bench_result.median));

    let total_runtime: Duration = benchmarked
        .iter()
        .map(|(_, bench_result)| bench_result.median)
        .sum();
    println!("{}: {:.2?}", highlighted("Total runtime"), total_runtime);
    println!("{}:", highlighted("Solutions (slowest to fastest)"));
    for (puzzle_id, bench_result) in benchmarked {
        println!("  {}: {:#}", highlighted(puzzle_id), bench_result);
    }

    Ok(())
}

fn benchmark(puzzle_id: PuzzleId, solution: Solution, min_time: Duration) -> Result<BenchResult> {
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
        let median = FormattedRuntime(self.median);
        let min = FormattedRuntime(self.min);
        let max = FormattedRuntime(self.max);
        let runs = self.runs;

        // Alternate switches to table format
        let w = if f.alternate() { 8 } else { 0 };
        write!(f, "{median:w$} ({min:w$} .. {max:w$}, {runs} runs)")
    }
}

struct FormattedRuntime(Duration);

impl Display for FormattedRuntime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        write!(f, "{:>width$.2?}", self.0)
    }
}
