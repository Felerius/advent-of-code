use anyhow::{bail, Result};

pub(crate) fn run(input: &str) -> Result<(u32, u32)> {
    let mut coord = HexCoord(0, 0);
    let mut part2 = coord.distance_to_origin();
    for dir in input.split(',') {
        let HexCoord(q, r) = coord;
        coord = match dir {
            "n" => HexCoord(q, r - 1),
            "ne" => HexCoord(q + 1, r - 1),
            "se" => HexCoord(q + 1, r),
            "s" => HexCoord(q, r + 1),
            "sw" => HexCoord(q - 1, r + 1),
            "nw" => HexCoord(q - 1, r),
            _ => bail!("Invalid direction: {dir:?}"),
        };
        part2 = part2.max(coord.distance_to_origin());
    }

    let part1 = coord.distance_to_origin();
    Ok((part1, part2))
}

/// Based on: <https://www.redblobgames.com/grids/hexagons/>
#[derive(Debug, Clone, Copy)]
struct HexCoord(i32, i32);

impl HexCoord {
    fn distance_to_origin(self) -> u32 {
        let HexCoord(q, r) = self;
        (q.unsigned_abs() + r.unsigned_abs() + (q + r).unsigned_abs()) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((3, 3), run("ne,ne,ne").unwrap());
        assert_eq!((0, 2), run("ne,ne,sw,sw").unwrap());
        assert_eq!((2, 2), run("ne,ne,s,s").unwrap());
        assert_eq!((3, 3), run("se,sw,se,sw,sw").unwrap());
    }
}
