use std::{iter, str::FromStr};

use anyhow::{Context, Result};
use itertools::izip;
use register::register;

#[register]
fn run(input: &str) -> Result<(u64, u64)> {
    let mut lines = input.lines();
    let operator_line = lines.next_back().context("empty input")?;
    let mut part1_aggs: Vec<_> = operator_line
        .bytes()
        .filter(|&b| !b.is_ascii_whitespace())
        .map(|b| Aggregator::new(b == b'*'))
        .collect();
    let mut vertical_values = vec![None; operator_line.len()];
    for line in lines {
        vertical_values.resize(line.len().max(vertical_values.len()), None);
        for (b, val) in izip!(line.bytes(), &mut vertical_values) {
            if b.is_ascii_digit() {
                *val = Some(val.unwrap_or(0) * 10 + u64::from(b - b'0'));
            }
        }

        let line_values = line.split_ascii_whitespace().map(u64::from_str);
        for (value, agg) in izip!(line_values, &mut part1_aggs) {
            agg.push(value?);
        }
    }

    let part1 = part1_aggs.iter().map(|agg| agg.1).sum();
    let extended_operator_line = operator_line.bytes().chain(iter::repeat(b' '));
    let mut cur_agg = Aggregator::new(false);
    let mut part2 = 0;
    for (maybe_value, b) in vertical_values.into_iter().zip(extended_operator_line) {
        if !b.is_ascii_whitespace() {
            part2 += cur_agg.1;
            cur_agg = Aggregator::new(b == b'*');
        }
        if let Some(value) = maybe_value {
            cur_agg.push(value);
        }
    }
    part2 += cur_agg.1;

    Ok((part1, part2))
}

#[derive(Debug, Clone, Copy)]
struct Aggregator(bool, u64);

impl Aggregator {
    fn new(is_mul: bool) -> Self {
        Self(is_mul, u64::from(is_mul))
    }

    fn push(&mut self, value: u64) {
        if self.0 {
            self.1 *= value;
        } else {
            self.1 += value;
        }
    }
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
