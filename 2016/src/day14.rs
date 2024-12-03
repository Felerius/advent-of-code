use std::{collections::VecDeque, sync::mpsc, thread};

use anyhow::{anyhow, Result};
use panic_message::panic_message;
use utils::md5::{Digest, SingleBlock};

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut searcher1 = Searcher::new();
    for n in 0.. {
        if n >= searcher1.stop_bound {
            break;
        }

        searcher1.consume(n, block_for_num(input, n).digest());
    }

    let mut searcher2 = Searcher::new();
    let num_threads = num_cpus::get();
    thread::scope(|scope| {
        let (senders, receivers): (Vec<_>, Vec<_>) =
            (0..num_threads).map(|_| mpsc::channel()).unzip();
        let handles: Vec<_> = senders
            .into_iter()
            .enumerate()
            .map(|(i, sender)| {
                scope.spawn(move || {
                    for n in (i..).step_by(num_threads) {
                        let digest = calc_stretched_hash(input, n);
                        if sender.send(digest).is_err() {
                            break;
                        }
                    }
                })
            })
            .collect();

        for n in 0.. {
            if n >= searcher2.stop_bound {
                break;
            }

            let digest = receivers[n % num_threads].recv()?;
            searcher2.consume(n, digest);
        }

        drop(receivers);
        handles.into_iter().try_for_each(|handle| {
            handle
                .join()
                .map_err(|p| anyhow!("search thread panicked: {}", panic_message(&p)))
        })
    })?;

    Ok((searcher1.into_ans(), searcher2.into_ans()))
}

fn block_for_num(input: &str, mut num: usize) -> SingleBlock {
    let digits = if num == 0 {
        1
    } else {
        num.ilog10() as usize + 1
    };

    let mut block = SingleBlock::new(input.len() + digits);
    block[..input.len()].copy_from_slice(input.as_bytes());
    for i in 0..digits {
        block[input.len() + digits - 1 - i] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    block
}

fn calc_stretched_hash(input: &str, n: usize) -> Digest {
    let mut block = block_for_num(input, n);
    let mut digest = block.digest();

    block = SingleBlock::new(32);
    for _ in 0..2016 {
        for (i, byte) in digest_digits(digest).enumerate() {
            block[i] = if byte < 10 {
                b'0' + byte
            } else {
                b'a' + byte - 10
            };
        }

        digest = block.digest();
    }

    digest
}

fn digest_digits(digest: Digest) -> impl Iterator<Item = u8> {
    digest
        .into_iter()
        .flat_map(|block| (0..8).rev().map(move |i| (block >> (i * 4) & 0xF) as u8))
}

struct Searcher {
    found: Vec<usize>,
    stop_bound: usize,
    candidates: VecDeque<(usize, u8)>,
}

impl Searcher {
    fn new() -> Self {
        Self {
            found: Vec::new(),
            stop_bound: usize::MAX,
            candidates: VecDeque::new(),
        }
    }

    fn consume(&mut self, n: usize, digest: Digest) {
        let mut prev = 16;
        let mut cnt = 0;
        let mut triple = None;
        let mut quintuples = 0;
        for digit in digest_digits(digest) {
            if digit == prev {
                cnt += 1;
                if cnt == 3 {
                    triple = triple.or(Some(digit));
                } else if cnt == 5 {
                    quintuples |= 1 << digit;
                }
            } else {
                prev = digit;
                cnt = 1;
            }
        }

        if quintuples != 0 {
            while self.candidates.front().is_some_and(|(c, _)| n - c > 1000) {
                self.candidates.pop_front();
            }

            self.candidates.retain(|&(c, c_triple)| {
                if quintuples >> c_triple & 1 != 0 {
                    self.found.push(c);
                    if self.found.len() >= 64 {
                        self.stop_bound = self.stop_bound.min(c + 1000);
                    }

                    false
                } else {
                    true
                }
            });
        }

        if let Some(triple) = triple {
            self.candidates.push_back((n, triple));
        }
    }

    fn into_ans(mut self) -> usize {
        self.found.sort_unstable();
        self.found[63]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (part1, part2) = run("abc").unwrap();
        assert_eq!(part1, 22728);
        assert_eq!(part2, 22551);
    }
}
