use anyhow::Result;
use num::integer::Roots;

const BLOCK: usize = 1 << 16;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let target: usize = input.parse()?;
    let mut data = Box::new([0; BLOCK]);
    let part1 = part1(target.div_ceil(10), &mut data);
    let part2 = part2(target.div_ceil(11), &mut data);
    Ok((part1, part2))
}

fn part1(target: usize, sieve: &mut [usize; BLOCK]) -> usize {
    let mut small_sieve = SmallSieve::new();
    for block_start in (lower_bound(target)..).step_by(BLOCK) {
        sieve.fill(1);
        small_sieve.extend_to((block_start + BLOCK).sqrt());
        for &p in &small_sieve.primes {
            let first = block_start.next_multiple_of(p).max(p) - block_start;
            for i in (first..BLOCK).step_by(p) {
                let mut p_pow = p * p;
                while (block_start + i) % p_pow == 0 {
                    p_pow *= p;
                }
                sieve[i] *= (p_pow - 1) / (p - 1);
            }
        }

        for i in 0..BLOCK {
            if sieve[i] == 1 {
                sieve[i] = block_start + i + 1;
            }
            if sieve[i] >= target {
                return block_start + i;
            }
        }
    }

    unreachable!("endless loop finished")
}

fn part2(target: usize, dp: &mut [usize; BLOCK]) -> usize {
    for block_start in (lower_bound(target)..).step_by(BLOCK) {
        for i in 0..BLOCK {
            dp[i] = i + block_start;
        }

        for i in BLOCK..(block_start + BLOCK) / 2 {
            let j = block_start.next_multiple_of(i);
            if let Some(entry) = dp.get_mut(j - block_start) {
                *entry += i;
            }
        }

        for i in block_start.div_ceil(50)..BLOCK {
            let low = block_start.next_multiple_of(i) - block_start;
            let high = (50 * i + 1 - block_start).min(BLOCK);
            for j in (low..high).step_by(i) {
                dp[j] += i;
            }
        }

        for i in 0..BLOCK {
            if dp[i] >= target {
                return block_start + i;
            }
        }
    }

    unreachable!("endless loop finished")
}

fn lower_bound(target: usize) -> usize {
    let target = target as f64;
    let mut high = 1;
    while robins_upper_bound(high) <= target {
        high *= 2;
    }
    let mut low = high / 2;
    while high - low > 1 {
        let mid = (low + high) / 2;
        if robins_upper_bound(mid) <= target {
            low = mid;
        } else {
            high = mid;
        }
    }

    high
}

fn robins_upper_bound(n: usize) -> f64 {
    // See https://en.wikipedia.org/wiki/Divisor_function#Growth_rate
    const E_TO_GAMMA: f64 = 1.7810724179901979852365041031071795491696452143034302053576658765;
    let n = n as f64;
    E_TO_GAMMA * n * n.ln().ln()
}

struct SmallSieve {
    primes: Vec<usize>,
    sieve: Vec<bool>,
}

impl SmallSieve {
    fn new() -> Self {
        Self {
            primes: Vec::new(),
            sieve: vec![false; 2],
        }
    }

    fn extend_to(&mut self, end: usize) {
        let first_new = self.sieve.len();
        self.sieve.resize(end, true);
        for &p in &self.primes {
            let first = first_new.div_ceil(p) * p;
            for i in (first..end).step_by(p) {
                self.sieve[i] = false;
            }
        }
        for i in first_new..end {
            if self.sieve[i] {
                self.primes.push(i);
                for j in (i * i..end).step_by(i) {
                    self.sieve[j] = false;
                }
            }
        }
    }
}
