use std::{
    collections::BinaryHeap,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
    thread,
};

use anyhow::{anyhow, Result};
use panic_message::panic_message;
use utils::md5::SingleBlock;

const BLOCK_SIZE: usize = 1000;

pub(crate) fn run(input: &str) -> Result<(String, String)> {
    let state = State::new();
    for num in 1..BLOCK_SIZE {
        state.consume(num, &prepare_block(input, num));
    }

    thread::scope(|scope| {
        let handles: Vec<_> = (0..num_cpus::get())
            .map(|_| scope.spawn(|| search_thread(input, &state)))
            .collect();
        handles.into_iter().try_for_each(|handle| {
            handle
                .join()
                .map_err(|p| anyhow!("search thread panicked: {}", panic_message(&p)))
        })
    })?;

    let solution = state.solution.into_inner().unwrap();
    let part1_digits = solution.part1.into_sorted_vec().into_iter().map(|(_, d)| d);
    let part2_digits = solution.part2.map(|(_, d)| d);

    Ok((to_hex_str(part1_digits), to_hex_str(part2_digits)))
}

fn to_hex_str(digits: impl IntoIterator<Item = u8>) -> String {
    let num = digits.into_iter().fold(0_u32, |x, d| x << 4 | u32::from(d));
    format!("{num:08x}")
}

fn prepare_block(input: &str, mut num: usize) -> SingleBlock {
    let digits = num.ilog10() as usize + 1;
    let mut block = SingleBlock::new(input.len() + digits);
    block[..input.len()].copy_from_slice(input.as_bytes());
    for i in 0..digits {
        block[input.len() + digits - 1 - i] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    block
}

fn search_thread(input: &str, state: &State) {
    while !state.stop.load(Ordering::Relaxed) {
        let block_start = state.next_block.fetch_add(BLOCK_SIZE, Ordering::Relaxed);
        let block_end = block_start + BLOCK_SIZE;
        let num_digits = block_start.ilog10() as usize + 1;
        debug_assert_eq!(num_digits, (block_end - 1).ilog10() as usize + 1);

        let mut block = prepare_block(input, block_start);
        for num in block_start..block_end {
            debug_assert_eq!(block, prepare_block(input, num));
            state.consume(num, &block);

            for i in (input.len()..input.len() + num_digits).rev() {
                if block[i] == b'9' {
                    block[i] = b'0';
                } else {
                    block[i] += 1;
                    break;
                }
            }
        }
    }
}

struct State {
    stop: AtomicBool,
    next_block: AtomicUsize,
    solution: Mutex<Solution>,
}

impl State {
    fn new() -> Self {
        Self {
            stop: AtomicBool::new(false),
            next_block: AtomicUsize::new(BLOCK_SIZE),
            solution: Mutex::new(Solution {
                part1: BinaryHeap::with_capacity(9),
                part2: [(usize::MAX, u8::MAX); 8],
                part2_cnt: 0,
            }),
        }
    }

    fn consume(&self, num: usize, block: &SingleBlock) {
        let digest = block.digest();
        if digest[0] & 0xFF_FF_F0_00 == 0 {
            let mut solution = self.solution.lock().unwrap();
            let digit6 = (digest[0] >> 8 & 0xF) as u8;
            let digit7 = (digest[0] >> 4 & 0xF) as u8;

            solution.part1.push((num, digit6));
            if solution.part1.len() == 9 {
                solution.part1.pop();
            }

            if digit6 < 8 {
                let lowest = &mut solution.part2[usize::from(digit6)];
                let new_char = lowest.0 == usize::MAX;
                *lowest = (*lowest).min((num, digit7));
                if new_char {
                    solution.part2_cnt += 1;
                    if solution.part2_cnt == 8 {
                        self.stop.store(true, Ordering::Relaxed);
                    }
                }
            }
        }
    }
}

struct Solution {
    part1: BinaryHeap<(usize, u8)>,
    part2: [(usize, u8); 8],
    part2_cnt: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (part1, part2) = run("abc").unwrap();
        assert_eq!(part1, "18f47a30");
        assert_eq!(part2, "05ace8e3");
    }
}
