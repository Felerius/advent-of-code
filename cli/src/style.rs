use std::{fmt::Display, time::Duration};

use console::{Color, StyledObject};
use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn highlighted<D: Display>(text: D) -> StyledObject<D> {
    console::style(text).blue().bold()
}

pub(crate) fn aoc_star() -> StyledObject<&'static str> {
    // color 227 is #ffff5f, which is quite close to the #ffff66 used on the website
    console::style("*").fg(Color::Color256(227))
}

pub(crate) fn spinner(message: &str, indent: usize) -> ProgressBar {
    let style = ProgressStyle::default_spinner()
        .template("{prefix}{msg}{spinner}")
        .expect("error in hardcoded progress bar template")
        .tick_strings(&["", ".", "..", "...", "..."]);
    let spinner = ProgressBar::new_spinner()
        .with_style(style)
        .with_prefix(" ".repeat(indent))
        .with_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(250));
    spinner
}
