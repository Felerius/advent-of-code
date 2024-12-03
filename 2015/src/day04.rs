use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
};

use anyhow::{anyhow, Result};
use panic_message::panic_message;
use utils::md5::SingleBlock;

const BLOCK_SIZE: usize = 1000;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let state = State::new();
    for num in 1..BLOCK_SIZE {
        let bytes = prepare_block(input, num);
        state.consume(num, &bytes);
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

    Ok((
        state.part1.load(Ordering::SeqCst),
        state.part2.load(Ordering::SeqCst),
    ))
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
            if state.consume(num, &block) {
                break;
            }

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
    part1: AtomicUsize,
    part2: AtomicUsize,
    stop: AtomicBool,
    next_block: AtomicUsize,
}

impl State {
    fn new() -> Self {
        Self {
            part1: AtomicUsize::new(usize::MAX),
            part2: AtomicUsize::new(usize::MAX),
            stop: AtomicBool::new(false),
            next_block: AtomicUsize::new(BLOCK_SIZE),
        }
    }

    fn consume(&self, num: usize, block: &SingleBlock) -> bool {
        let digest = block.digest();
        if digest[0] & 0xFF_FF_F0_00 == 0 {
            self.part1.fetch_min(num, Ordering::SeqCst);
            if digest[0] & 0xFF_FF_FF_00 == 0 {
                self.part2.fetch_min(num, Ordering::SeqCst);
                self.stop.store(true, Ordering::Relaxed);
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(run("abcdef")?.0, 609043);
        assert_eq!(run("pqrstuv")?.0, 1048970);
        Ok(())
    }
}
