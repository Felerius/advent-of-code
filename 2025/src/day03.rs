use arrayvec::ArrayVec;
use register::register;

#[register]
fn run(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            (choose_greedy::<2>(line), choose_greedy::<12>(line))
        })
        .fold((0, 0), |(part1, part2), (num1, num2)| {
            (part1 + num1, part2 + num2)
        })
}

fn choose_greedy<const N: usize>(line: &[u8]) -> u64 {
    let mut digits = ArrayVec::<_, N>::new();
    for (i, &d) in line.iter().enumerate() {
        let min_needed = N.saturating_sub(line.len() - i);
        while digits.len() > min_needed
            && let Some(&last) = digits.last()
            && d > last
        {
            digits.pop();
        }

        if digits.len() < N {
            digits.push(d);
        }
    }

    debug_assert_eq!(digits.len(), N);
    digits
        .into_iter()
        .fold(0, |num, d| num * 10 + u64::from(d - b'0'))
}

#[expect(dead_code, reason = "alternative implementation")]
fn choose_greedy_slow(mut line: &[u8], num: usize) -> u64 {
    let mut res = 0;
    for i in 0..num {
        let remaining = num - i - 1;
        let candidates = &line[..line.len() - remaining];
        // `max_by_key` returns the last maximum, so by using `rev` we get the first
        let (index, &digit) = candidates
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, d)| d)
            .unwrap();
        res = res * 10 + u64::from(digit - b'0');
        line = &line[index + 1..];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (357, 3_121_910_778_619));
    }
}
