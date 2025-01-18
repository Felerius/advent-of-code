use anyhow::{bail, Context, Result};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let mut part1 = None;
    let mut part2 = None;
    for (i, line) in input.lines().enumerate() {
        let line = line.split_once(": ").context("invalid input")?.1;
        let (match1, match2) = line
            .split(", ")
            .try_fold((true, true), |(match1, match2), s| {
                let (typ, value) = s.split_once(": ").context("invalid input")?;
                let value: u32 = value.parse().context("invalid input")?;
                let (valid1, valid2) = match typ {
                    "children" => (value == 3, value == 3),
                    "cats" => (value == 7, value > 7),
                    "samoyeds" | "cars" => (value == 2, value == 2),
                    "pomeranians" => (value == 3, value < 3),
                    "akitas" | "vizslas" => (value == 0, value == 0),
                    "goldfish" => (value == 5, value < 5),
                    "trees" => (value == 3, value > 3),
                    "perfumes" => (value == 1, value == 1),
                    _ => bail!("unknown property: {typ}"),
                };
                Ok((match1 && valid1, match2 && valid2))
            })?;

        if match1 {
            part1 = Some(i + 1);
        }
        if match2 {
            part2 = Some(i + 1);
        }
    }

    part1
        .zip(part2)
        .context("at least one part has no solution")
}
