use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

use console::{Color, Style};
use indicatif::{ProgressBar, ProgressStyle};

use crate::PuzzleId;

pub(crate) const HIGHLIGHT: Style = Style::new().blue().bold();
pub(crate) const DIM: Style = Style::new().dim();
pub(crate) const CORRECT: Style = Style::new().green();
pub(crate) const INCORRECT: Style = Style::new().red();

// color 227 is #ffff5f, which is quite close to the #ffff66 used on the website
pub(crate) const AOC_STAR: StyledStaticStr =
    StyledStaticStr("*", Style::new().fg(Color::Color256(227)));
pub(crate) const CHECKMARK: StyledStaticStr = StyledStaticStr("✓", CORRECT.bold());
pub(crate) const CROSSMARK: StyledStaticStr = StyledStaticStr("✗", INCORRECT.bold());

pub(crate) fn progress_style() -> ProgressStyle {
    ProgressStyle::default_bar().progress_chars("█▉▊▋▌▍▎▏  ")
}

pub(crate) fn print_runtime_bar(puzzle_id: PuzzleId, runtime: Duration, max_runtime: Duration) {
    let style = progress_style()
        .template("  {prefix} {wide_bar:.dim} {msg}")
        .expect("error in hardcoded progress bar template");
    let max_runtime_nanos: u64 = max_runtime
        .as_nanos()
        .try_into()
        .expect("runtime over 500 years");
    let runtime_nanos: u64 = runtime
        .as_nanos()
        .try_into()
        .expect("runtime over 500 years");

    let prefix = format!("{AOC_STAR} {}", HIGHLIGHT.apply_to(puzzle_id));
    let bar = ProgressBar::new(max_runtime_nanos)
        .with_style(style)
        .with_prefix(prefix)
        .with_message(format!("{runtime:>8.2?}"));
    bar.set_position(runtime_nanos);
    bar.abandon();
}

pub(crate) struct StyledStaticStr(&'static str, Style);

impl Display for StyledStaticStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1.apply_to(self.0))
    }
}
