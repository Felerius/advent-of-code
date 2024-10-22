use std::{
    array,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use anyhow::{anyhow, Result};
use md5::{Context, Digest};
use panic_message::panic_message;

const BLOCK_SIZE: usize = 1000;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut context = Context::new();
    context.consume(input);
    let stop_signal = AtomicBool::new(false);
    thread::scope(|scope| {
        let stop_handles: [_; 10] = array::from_fn(|i| {
            let context = context.clone();
            let stop_signal = &stop_signal;
            scope.spawn(move || search_thread(i as u8, context, stop_signal))
        });
        stop_handles
            .into_iter()
            .map(|handle| {
                handle.join().map_err(|payload| {
                    anyhow!("search thread panicked: {}", panic_message(&payload))
                })
            })
            .try_fold((usize::MAX, usize::MAX), |(part1, part2), res| {
                let (part1_thread, part2_thread) = res?;
                Ok((part1.min(part1_thread), part2.min(part2_thread)))
            })
    })
}

fn search_thread(last_digit: u8, mut context: Context, stop_signal: &AtomicBool) -> (usize, usize) {
    let mut part1 = None;
    let mut digits = [0; 9];
    let mut contexts: [_; 9] = array::from_fn(|_| context.clone());
    let mut block_start = 1;

    while !stop_signal.load(Ordering::Relaxed) {
        for i in block_start..block_start + BLOCK_SIZE {
            for j in (0..9).rev() {
                if digits[j] < 9 {
                    context = contexts[j - 1].clone();
                    digits[j] += 1;
                    for k in j..9 {
                        context.consume(&[b'0' + digits[k]]);
                        contexts[k] = context.clone();
                    }
                    break;
                }

                digits[j] = 0;
            }

            context.consume(&[b'0' + last_digit]);
            let Digest(bytes) = context.clone().compute();
            if bytes[0] == 0 && bytes[1] == 0 && (bytes[2] >> 4) == 0 {
                let num = i * 10 + usize::from(last_digit);
                let part1_known = *part1.get_or_insert(num);

                if bytes[2] == 0 {
                    stop_signal.store(true, Ordering::Relaxed);
                    return (part1_known, num);
                }
            }
        }

        block_start += BLOCK_SIZE;
    }

    (part1.unwrap_or(usize::MAX), usize::MAX)
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
