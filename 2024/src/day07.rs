use array_const_fn_init::array_const_fn_init;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn run(input: &str) -> (u64, u64) {
    let lines: Vec<_> = input.lines().collect();
    lines
        .into_par_iter()
        .map(|line| {
            let (total, nums) = line.split_once(": ").expect("invalid line");
            let total: u64 = total.parse().expect("invalid total");
            let nums: Vec<_> = nums
                .split(' ')
                .map(|n| (n.parse().expect("invalid num"), n.len()))
                .collect();

            let (part1_valid, part2_valid) = enumerate(total, &nums);
            let part1 = if part1_valid { total } else { 0 };
            let part2 = if part2_valid { total } else { 0 };

            (part1, part2)
        })
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
}

fn enumerate(rem: u64, nums: &[(u64, usize)]) -> (bool, bool) {
    let (&(num, num_digits), nums) = nums.split_last().unwrap();
    if nums.is_empty() {
        return (rem == num, rem == num);
    }

    let mut part1 = false;
    let mut part2 = false;
    if rem % num == 0 {
        (part1, part2) = enumerate(rem / num, nums);
    }
    if !part2 && rem % TEN_POW[num_digits] == num {
        part2 |= enumerate(rem / TEN_POW[num_digits], nums).1;
    }
    if !part1 && rem >= num {
        let (cpart1, cpart2) = enumerate(rem - num, nums);
        part1 |= cpart1;
        part2 |= cpart2;
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
        assert_eq!(run(INPUT), (3749, 11387));
    }
}
