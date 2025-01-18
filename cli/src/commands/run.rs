use anyhow::{Context, Result};
use collect::Output;

use crate::{
    commands::PuzzleArgs,
    inputs,
    style::{aoc_star, highlighted, spinner},
};

pub(crate) type Args = PuzzleArgs;

pub(crate) fn run(args: &Args) -> Result<()> {
    for (puzzle_id, solution) in args.selected_puzzles()? {
        println!("{}", highlighted(puzzle_id));
        let spinner = spinner("running", 2);

        let input = inputs::get(puzzle_id)?;
        let Output { part1, part2 } =
            solution(&input).with_context(|| format!("solution for {puzzle_id} failed"))?;
        spinner.finish_and_clear();

        let star = aoc_star();
        println!("  {star} Part 1: {part1}");
        if puzzle_id.day != 25 {
            println!("  {star} Part 2: {part2}");
        }
    }

    Ok(())
}
