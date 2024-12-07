use anyhow::Result;
use array_const_fn_init::array_const_fn_init;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let lines: Vec<_> = input.lines().collect();
    let (part1, part2) = lines
        .into_par_iter()
        .map(|line| {
            let (total, nums) = line.split_once(": ").expect("invalid line");
            let total: u64 = total.parse().expect("invalid total");
            let nums: Vec<_> = nums
                .split(' ')
                .map(|n| n.parse().expect("invalid num"))
                .collect();

            let (part1_valid, part2_valid) = enumerate(Some(0), &nums, total);
            let part1 = if part1_valid { total } else { 0 };
            let part2 = if part2_valid { total } else { 0 };

            (part1, part2)
        })
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));

    Ok((part1, part2))
}

fn enumerate(val: Option<u64>, nums: &[u64], target: u64) -> (bool, bool) {
    let Some(val) = val else {
        return (false, false);
    };
    let Some((&i, rem_nums)) = nums.split_first() else {
        return (val == target, val == target);
    };

    let (mut part1, mut part2) = enumerate(val.checked_add(i), rem_nums, target);
    if !part2 {
        let (cpart1, cpart2) = enumerate(val.checked_mul(i), rem_nums, target);
        part1 |= cpart1;
        part2 |= cpart2;
    }
    if !part2 {
        let i_digits = i.checked_ilog10().unwrap_or(1) as usize;
        let val2 = val.checked_mul(TEN_POW[i_digits]).map(|v| v + i);
        part2 |= enumerate(val2, rem_nums, target).1;
    }

    (part1, part2)
}

const TEN_POW: [u64; 20] = array_const_fn_init![ten_pow; 20];

const fn ten_pow(i: usize) -> u64 {
    10_u64.pow(i as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (3749, 11387));
    }
}
