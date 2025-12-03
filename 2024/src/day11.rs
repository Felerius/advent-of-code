use std::{array, mem};

use anyhow::Result;
use const_array_init::const_arr;
use utils::hash::{FastHashCollectionExt, FastHashMap};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let mut cur_cnts = FastHashMap::<_, usize>::with_capacity(4096);
    let mut prev_cnts = FastHashMap::with_capacity(4096);
    for s in input.split_ascii_whitespace() {
        let num: u64 = s.parse()?;
        *cur_cnts.entry(num).or_default() += 1;
    }

    let totals: [_; 75] = array::from_fn(|_| {
        mem::swap(&mut prev_cnts, &mut cur_cnts);
        cur_cnts.clear();
        simulate_step(&prev_cnts, &mut cur_cnts)
    });

    Ok((totals[24], totals[74]))
}

fn simulate_step(
    prev_cnts: &FastHashMap<u64, usize>,
    cur_cnts: &mut FastHashMap<u64, usize>,
) -> usize {
    prev_cnts
        .iter()
        .map(|(&num, &cnt)| {
            if num == 0 {
                *cur_cnts.entry(1).or_default() += cnt;
                cnt
            } else if num.ilog10() % 2 == 1 {
                let len = num.ilog10() as usize + 1;
                *cur_cnts.entry(num / TEN_POW[len / 2]).or_default() += cnt;
                *cur_cnts.entry(num % TEN_POW[len / 2]).or_default() += cnt;
                2 * cnt
            } else {
                *cur_cnts.entry(2024 * num).or_default() += cnt;
                cnt
            }
        })
        .sum()
}

const TEN_POW: [u64; 20] = const_arr!([u64; 20], |i| 10_u64.pow(i as u32));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("125 17").unwrap().0, 55312);
    }
}
