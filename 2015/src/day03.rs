use anyhow::{bail, Result};
use rustc_hash::FxHashSet;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut pos1 = (0, 0);
    let mut pos2 = [(0, 0), (0, 0)];
    let mut part1 = FxHashSet::from_iter([(0, 0)]);
    let mut part2 = part1.clone();
    for c in input.bytes() {
        pos1 = apply_move(pos1, c)?;
        part1.insert(pos1);
        pos2 = [pos2[1], apply_move(pos2[0], c)?];
        part2.insert(pos2[1]);
    }

    Ok((part1.len(), part2.len()))
}

fn apply_move(pos: (i32, i32), c: u8) -> Result<(i32, i32)> {
    let pos = match c {
        b'^' => (pos.0, pos.1 + 1),
        b'v' => (pos.0, pos.1 - 1),
        b'>' => (pos.0 + 1, pos.1),
        b'<' => (pos.0 - 1, pos.1),
        _ => bail!("invalid move: {}", c as char),
    };
    Ok(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let inputs = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];
        for (input, expected) in inputs {
            assert_eq!(run(input)?.0, expected, "failed for {input:?}");
        }
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let inputs = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];
        for (input, expected) in inputs {
            assert_eq!(run(input)?.1, expected, "failed for {input:?}");
        }
        Ok(())
    }
}
