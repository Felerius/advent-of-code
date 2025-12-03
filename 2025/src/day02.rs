use const_array_init::const_arr;
use register::register;

const TEN_POW: [u64; 20] = const_arr!([u64; 20], |i| 10_u64.pow(i as u32));
const MOEBIUS: [i64; 20] = [
    0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, 1, 1, -1, 0, -1, 1, -1,
];

#[register]
fn run(input: &str) -> (u64, i64) {
    let ranges = input.split(',').flat_map(|r| {
        let (l, r) = r.split_once('-').unwrap();
        let l: u64 = l.parse().unwrap();
        let r: u64 = r.parse().unwrap();
        split_range(l, r)
    });

    let mut part1 = 0;
    let mut part2 = 0;
    for (low, high, digits) in ranges {
        if digits.is_multiple_of(2) {
            part1 += sum_of_nums(low, high, 2, digits / 2);
        }

        let divs = (2..=digits).filter(|&d| digits.is_multiple_of(d));
        for rep in divs {
            if MOEBIUS[rep] != 0 {
                part2 -= MOEBIUS[rep] * sum_of_nums(low, high, rep, digits / rep) as i64;
            }
        }
    }

    (part1, part2)
}

fn split_range(low: u64, high: u64) -> impl Iterator<Item = (u64, u64, usize)> {
    (num_digits(low)..=num_digits(high)).map(move |digits| {
        let split_low = TEN_POW[digits - 1].max(low);
        let split_high = (TEN_POW[digits] - 1).min(high);
        (split_low, split_high, digits)
    })
}

fn sum_of_nums(low: u64, high: u64, rep: usize, digits: usize) -> u64 {
    let total_digits = rep * digits;
    debug_assert_eq!(num_digits(low), rep * digits);
    debug_assert_eq!(num_digits(high), rep * digits);

    let repeat_factor = (TEN_POW[total_digits] - 1) / (TEN_POW[digits] - 1);
    let mut low_block = low / TEN_POW[total_digits - digits];
    if low_block * repeat_factor < low {
        low_block += 1;
    }

    let mut high_block = high / TEN_POW[total_digits - digits];
    if high_block * repeat_factor > high {
        high_block -= 1;
    }

    if low_block <= high_block {
        let sum_to_high = high_block * (high_block + 1) / 2;
        let sum_below_low = (low_block - 1) * low_block / 2;
        (sum_to_high - sum_below_low) * repeat_factor
    } else {
        0
    }
}

fn num_digits(x: u64) -> usize {
    x.ilog10() as usize + 1
}

#[expect(dead_code, reason = "alternative implementation")]
fn is_silly_slow(x: u64) -> bool {
    let digits = num_digits(x);
    let ten_pow = TEN_POW[digits / 2];
    digits.is_multiple_of(2) && x / ten_pow == x % ten_pow
}

#[expect(dead_code, reason = "alternative implementation")]
fn is_silly2_slow(x: u64) -> bool {
    let digits = num_digits(x);
    (2..=digits).filter(|&i| digits.is_multiple_of(i)).any(|i| {
        let w = digits / i;
        let ten_pow = TEN_POW[w];
        let first = x % ten_pow;
        (1..i).all(|j| first == (x / TEN_POW[j * w]) % ten_pow)
    })
}

#[expect(dead_code, reason = "alternative implementation")]
fn is_silly_super_slow(x: u64) -> bool {
    let s = x.to_string().into_bytes();
    s.len().is_multiple_of(2) && s[..s.len() / 2] == s[s.len() / 2..]
}
#[expect(dead_code, reason = "alternative implementation")]
fn is_silly2_super_slow(x: u64) -> bool {
    let s = x.to_string().into_bytes();
    (2..=s.len())
        .filter(|&x| s.len().is_multiple_of(x))
        .any(|x| {
            let w = s.len() / x;
            (1..x).all(|i| s[..w] == s[i * w..(i + 1) * w])
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (1_227_775_554, 4_174_379_265));
    }
}
