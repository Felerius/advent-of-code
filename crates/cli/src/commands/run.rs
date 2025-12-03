use anyhow::{Context, Result};

use crate::{
    Day,
    commands::MultiPuzzleArgs,
    inputs,
    style::{aoc_star, highlighted, spinner},
};

pub(crate) type Args = MultiPuzzleArgs;

pub(crate) fn run(args: &Args) -> Result<()> {
    for (puzzle_id, solution) in args.evaluate()? {
        println!("{}", highlighted(puzzle_id));
        let spinner = spinner("running", 2);

        let input = inputs::get(puzzle_id)?;
        let (part1, part2) =
            (solution.func)(&input).with_context(|| format!("solution for {puzzle_id} failed"))?;
        spinner.finish_and_clear();

        let star = aoc_star();
        println!("  {star} Part 1: {part1}");
        if puzzle_id.day != Day::TWENTY_FIVE {
            println!("  {star} Part 2: {part2}");
        }
    }

    Ok(())
}
