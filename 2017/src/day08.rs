use std::cmp::Ordering;

use anyhow::{Context, Result};
use itertools::Itertools;
use utils::hash::{FastHashCollectionExt, FastHashMap};

pub(crate) fn run(input: &str) -> Result<(i32, i32)> {
    let mut registers = FastHashMap::new();
    let mut part2 = 0;
    for line in input.lines() {
        let (reg, op, val, _, cond_reg, cond_op, cond_val) = line
            .split_ascii_whitespace()
            .collect_tuple()
            .context("invalid input line")?;

        let cond_reg_val = *registers.entry(cond_reg).or_insert(0);
        let cond_val: i32 = cond_val.parse()?;
        let cmp = cond_reg_val.cmp(&cond_val);
        let matching_ops = match cmp {
            Ordering::Less => ["<", "<=", "!="],
            Ordering::Equal => ["==", ">=", "<="],
            Ordering::Greater => [">", ">=", "!="],
        };
        if !matching_ops.contains(&cond_op) {
            continue;
        }

        let reg = registers.entry(reg).or_insert(0);
        let val: i32 = val.parse()?;
        if op == "inc" {
            *reg += val;
        } else {
            *reg -= val;
        }
        part2 = part2.max(*reg);
    }

    let part1 = registers.into_values().max().unwrap_or(0);
    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn test() {
        let (part1, part2) = run(INPUT).unwrap();
        assert_eq!(part1, 1);
        assert_eq!(part2, 10);
    }
}
