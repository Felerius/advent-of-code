use anyhow::{Context, Result};

use crate::{
    Day,
    commands::{MultiPuzzleArgs, init_progress_bar},
    inputs,
    style::{AOC_STAR, CHECKMARK, CORRECT, CROSSMARK, DIM, HIGHLIGHT, INCORRECT, StyledStaticStr},
};

#[derive(clap::Args)]
pub(crate) struct Args {
    #[clap(flatten)]
    puzzles: MultiPuzzleArgs,

    /// Run alternative solutions
    #[clap(long)]
    alts: bool,
}

pub(crate) fn run(args: &Args) -> Result<()> {
    let puzzles = args.puzzles.evaluate()?;
    let progress_bar = init_progress_bar(&puzzles, args.alts);
    for (puzzle_id, solution) in puzzles {
        let input = inputs::get(puzzle_id)?;
        let (part1, part2) =
            (solution.main)(&input).with_context(|| format!("solution for {puzzle_id} failed"))?;

        let results = if puzzle_id.day == Day::TWENTY_FIVE {
            part1.clone()
        } else {
            format!("{part1} {part2}")
        };
        let header = HIGHLIGHT.apply_to(format!("{puzzle_id}:"));
        let msg = format!("{AOC_STAR} {header} {results}");
        progress_bar.inc(1);
        progress_bar.println(msg);

        if args.alts {
            for (alt_name, alt_solution) in &solution.alts {
                let (alt_part1, alt_part2) = (*alt_solution)(&input).with_context(|| {
                    format!("alternative solution {alt_name:?} for {puzzle_id} failed")
                })?;

                let mark1 = check_or_cross(alt_part1 == part1);
                let mark2 = check_or_cross(alt_part2 == part2);
                let msg = format!("    {mark1} {mark2} {}", DIM.apply_to(alt_name));
                progress_bar.inc(1);
                progress_bar.println(msg);

                for (main, alt) in [(&part1, &alt_part1), (&part2, &alt_part2)] {
                    if main != alt {
                        let main = CORRECT.apply_to(main);
                        let alt = INCORRECT.apply_to(alt);
                        progress_bar.println(format!("        {main} vs. {alt}",));
                    }
                }
            }
        }
    }

    Ok(())
}

fn check_or_cross(correct: bool) -> &'static StyledStaticStr {
    if correct { &CHECKMARK } else { &CROSSMARK }
}
