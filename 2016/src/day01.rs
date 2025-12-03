use anyhow::{Context, Result};
use register::register;
use utils::hash::{FastHashCollectionExt, FastHashSet};

#[register]
fn run(input: &str) -> Result<(i16, i16)> {
    let mut pos = (0_i16, 0_i16);
    let mut dir = (0, 1);
    let mut seen = FastHashSet::with_capacity(1024);
    let mut part2 = None;
    seen.insert(pos);
    for instr in input.split(", ") {
        dir = if instr.starts_with('L') {
            (-dir.1, dir.0)
        } else {
            (dir.1, -dir.0)
        };
        let dist: i16 = instr[1..].parse().context("invalid instruction")?;

        if part2.is_none() {
            let mut pos2 = pos;
            for _ in 0..dist {
                pos2 = (pos2.0 + dir.0, pos2.1 + dir.1);
                if !seen.insert(pos2) {
                    part2 = Some(pos2.0.abs() + pos2.1.abs());
                    break;
                }
            }
        }

        pos = (pos.0 + dir.0 * dist, pos.1 + dir.1 * dist);
    }

    Ok((
        pos.0.abs() + pos.1.abs(),
        part2.context("no square visited twice")?,
    ))
}
