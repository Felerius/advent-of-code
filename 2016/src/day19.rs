use std::collections::VecDeque;

use anyhow::Result;
use register::register;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let n: usize = input.parse()?;

    // Josephus problem: https://en.wikipedia.org/wiki/Josephus_problem#Solution
    let part1 = (n ^ 1 << n.ilog2()) << 1 | 1;

    let pow3 = 3_usize.pow((n - 1).ilog(3));
    let part2 = n - pow3 + n.saturating_sub(2 * pow3);

    Ok((part1, part2))
}

#[allow(dead_code)]
fn simulate_part1(n: usize) -> usize {
    let mut queue: VecDeque<_> = (1..=n).collect();
    while let Some(x) = queue.pop_front() {
        if queue.pop_front().is_none() {
            return x;
        }
        queue.push_back(x);
    }

    unreachable!()
}

#[allow(dead_code)]
fn simulate_part2(n: usize) -> usize {
    let middle = n / 2 + 1;
    let mut front: VecDeque<_> = (1..middle).collect();
    let mut back: VecDeque<_> = (middle..=n).collect();
    while let Some(x) = front.pop_front() {
        back.pop_front();
        back.push_back(x);
        if front.len() + 1 < back.len() {
            front.push_back(back.pop_front().unwrap());
        }
    }

    back.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("5").unwrap().0, 3);
    }
}
