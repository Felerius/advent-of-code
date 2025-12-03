use std::{fmt::Display, time::Duration};

use console::{Color, StyledObject};
use indicatif::{ProgressBar, ProgressStyle};

use crate::PuzzleId;

pub(crate) fn highlighted<D: Display>(text: D) -> StyledObject<D> {
    console::style(text).blue().bold()
}

pub(crate) fn error<D: Display>(text: D) -> StyledObject<D> {
    console::style(text).red().bold()
}

pub(crate) fn aoc_star() -> StyledObject<&'static str> {
    // color 227 is #ffff5f, which is quite close to the #ffff66 used on the website
    console::style("*").fg(Color::Color256(227))
}

pub(crate) fn spinner(message: impl Into<String>, indent: usize) -> ProgressBar {
    let style = ProgressStyle::default_spinner()
        .template("{prefix}{msg}{spinner}")
        .expect("error in hardcoded progress bar template")
        .tick_strings(&["", ".", "..", "...", "..."]);
    let spinner = ProgressBar::new_spinner()
        .with_style(style)
        .with_prefix(" ".repeat(indent))
        .with_message(message.into());
    spinner.enable_steady_tick(Duration::from_millis(250));
    spinner
}

pub(crate) fn progress_style() -> ProgressStyle {
    ProgressStyle::default_bar().progress_chars("█▉▊▋▌▍▎▏  ")
}

pub(crate) fn print_runtime_bar(puzzle_id: PuzzleId, runtime: Duration, max_runtime: Duration) {
    let style = progress_style()
        .template("  {prefix:.bold.blue} {wide_bar:.dim} {msg}")
        .expect("error in hardcoded progress bar template");
    let max_runtime_nanos: u64 = max_runtime
        .as_nanos()
        .try_into()
        .expect("runtime over 500 years");
    let runtime_nanos: u64 = runtime
        .as_nanos()
        .try_into()
        .expect("runtime over 500 years");

    let bar = ProgressBar::new(max_runtime_nanos)
        .with_style(style)
        .with_prefix(puzzle_id.to_string())
        .with_message(format!("{runtime:>8.2?}"));
    bar.set_position(runtime_nanos);
    bar.abandon();
}
