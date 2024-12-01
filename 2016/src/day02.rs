use anyhow::{bail, Result};

const GRAPH1: &[(u8, [usize; 4])] = &[
    (b'1', [0, 0, 1, 3]),
    (b'2', [0, 1, 2, 4]),
    (b'3', [1, 2, 2, 5]),
    (b'4', [3, 0, 4, 6]),
    (b'5', [3, 1, 5, 7]),
    (b'6', [4, 2, 5, 8]),
    (b'7', [6, 3, 7, 6]),
    (b'8', [6, 4, 8, 7]),
    (b'9', [7, 5, 8, 8]),
];

const GRAPH2: &[(u8, [usize; 4])] = &[
    (b'1', [0, 0, 0, 2]),
    (b'2', [1, 1, 2, 5]),
    (b'3', [1, 0, 3, 6]),
    (b'4', [2, 3, 3, 7]),
    (b'5', [4, 4, 5, 4]),
    (b'6', [4, 1, 6, 9]),
    (b'7', [5, 2, 7, 10]),
    (b'8', [6, 3, 8, 11]),
    (b'9', [7, 8, 8, 8]),
    (b'A', [9, 5, 10, 9]),
    (b'B', [9, 6, 11, 12]),
    (b'C', [10, 7, 11, 11]),
    (b'D', [12, 10, 12, 12]),
];

pub fn run(input: &str) -> Result<(String, String)> {
    let mut v1 = 4;
    let mut v2 = 4;
    let mut part1 = String::new();
    let mut part2 = String::new();
    for line in input.lines() {
        for c in line.bytes() {
            let idx = match c {
                b'L' => 0,
                b'U' => 1,
                b'R' => 2,
                b'D' => 3,
                _ => bail!("Invalid direction: {}", char::from(c)),
            };
            v1 = GRAPH1[v1].1[idx];
            v2 = GRAPH2[v2].1[idx];
        }

        part1.push(char::from(GRAPH1[v1].0));
        part2.push(char::from(GRAPH2[v2].0));
    }

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let (actual1, actual2) = run(input).unwrap();
        assert_eq!("1985", actual1);
        assert_eq!("5DB3", actual2);
    }
}
