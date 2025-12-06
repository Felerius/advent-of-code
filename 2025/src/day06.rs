use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::izip;
use register::register;

#[register]
fn run(input: &str) -> Result<(u64, u64)> {
    let mut lines = input.lines();
    let operator_line = lines.next_back().context("empty input")?;
    let mut columns: Vec<_> = operator_line
        .as_bytes()
        .chunk_by(|_, &b| b.is_ascii_whitespace())
        .map(|chunk| (chunk[0] == b'*', chunk.len()))
        .collect();

    let mut vertical_values = vec![None; operator_line.len()];
    let mut part1_aggs: Vec<_> = columns
        .iter()
        .map(|(is_mul, _)| u64::from(*is_mul))
        .collect();
    for line in lines {
        if line.len() > vertical_values.len() {
            columns.last_mut().unwrap().1 += line.len() - vertical_values.len();
            vertical_values.resize(line.len(), None);
        }

        for (i, d) in line.bytes().enumerate() {
            if d.is_ascii_digit() {
                let digit = u64::from(d - b'0');
                vertical_values[i] = Some(vertical_values[i].unwrap_or(0) * 10 + digit);
            }
        }

        let line_values = line.split_ascii_whitespace().map(u64::from_str);
        for ((is_mut, _), agg, value) in izip!(&columns, &mut part1_aggs, line_values) {
            if *is_mut {
                *agg *= value?;
            } else {
                *agg += value?;
            }
        }
    }

    let part1 = part1_aggs.iter().sum();
    let mut offset = 0;
    let part2 = columns
        .iter()
        .map(|&(is_mul, width)| {
            let init = u64::from(is_mul);
            offset += width;
            vertical_values[offset - width..offset]
                .iter()
                .filter_map(|&v| v)
                .fold(init, |acc, v| if is_mul { acc * v } else { acc + v })
        })
        .sum();

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (4_277_556, 3_263_827));
    }
}
