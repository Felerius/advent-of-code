use anyhow::{Context, Result};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let part1 = decompressed_length(input, false)?;
    let part2 = decompressed_length(input, true)?;
    Ok((part1, part2))
}

fn decompressed_length(mut s: &str, recursive: bool) -> Result<usize> {
    let mut result = 0;
    while let Some(next_block) = s.find('(') {
        result += next_block;
        s = &s[next_block + 1..];

        let x_pos = s.find('x').context("invalid marker")?;
        let length: usize = s[..x_pos].parse().context("invalid marker")?;
        s = &s[x_pos + 1..];

        let end_pos = s.find(')').context("invalid marker")?;
        let repeat: usize = s[..end_pos].parse().context("invalid marker")?;
        s = &s[end_pos + 1..];

        result += repeat
            * if recursive {
                decompressed_length(&s[..length], true)?
            } else {
                length
            };
        s = &s[length..];
    }

    Ok(result + s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_recursive() {
        let inputs = [
            ("ADVENT", 6),
            ("A(1x5)BC", 7),
            ("(3x3)XYZ", 9),
            ("A(2x2)BCD(2x2)EFG", 11),
            ("(6x1)(1x3)A", 6),
            ("X(8x2)(3x3)ABCY", 18),
        ];
        for (input, expected) in inputs {
            assert_eq!(
                expected,
                decompressed_length(input, false).unwrap(),
                "failed for {input:?}"
            );
        }
    }

    #[test]
    fn recursive() {
        let inputs = [
            ("(3x3)XYZ", 9),
            ("X(8x2)(3x3)ABCY", 20),
            ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920),
            (
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                445,
            ),
        ];
        for (input, expected) in inputs {
            assert_eq!(
                expected,
                decompressed_length(input, true).unwrap(),
                "failed for {input:?}"
            );
        }
    }
}
