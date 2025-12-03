use std::{
    array, iter,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

use anyhow::Result;
use register::register;
use utils::input::Input;

const MOD: u64 = 2_147_483_647;
const F1: u64 = 16_807;
const F2: u64 = 48_271;
const N1: usize = 40_000_000;
const N2: usize = 5_000_000;
const CHUNK_SIZE: usize = 50_000;
const NUM_CHUNKS: usize = N1.checked_div(CHUNK_SIZE).unwrap();

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let [x, y] = input.unsigned_integers_n()?;
    Ok(parallel(x, y))
}

fn parallel(x: u64, y: u64) -> (usize, usize) {
    // The generated sequence is $x_{n + 1} = x_n * f \mod M$. Thus, in
    // general, $x_{n + m} = x_n * f^m \mod M$. This means we can jump ahead in
    // the sequence and generate different parts in parallel.
    //
    // Since this is a simple PRNG, we can assume that each bit is uniformly
    // distributed across values in each sequence. Thus, for part 2, we will
    // need about 20M values from the first generator and about 40M from the
    // second to get 5M valid pairs.
    let next_chunk = AtomicUsize::new(0);
    let (tx, rx) = mpsc::channel();
    thread::scope(|scope| {
        for _ in 0..num_cpus::get() - 1 {
            let tx = tx.clone();
            let next_chunk = &next_chunk;
            scope.spawn(move || producer(x, y, next_chunk, &tx));
        }
        drop(tx);
        consumer(x, y, &rx)
    })
}

fn producer(x0: u64, y0: u64, next_chunk: &AtomicUsize, tx: &Sender<(usize, ChunkResult)>) {
    loop {
        let chunk_index = next_chunk.fetch_add(1, Ordering::SeqCst);
        if chunk_index >= NUM_CHUNKS {
            break;
        }

        // let result = compute_chunk_loop(x0, y0, chunk_index);
        let result = compute_chunk_auto_vec(x0, y0, chunk_index);
        tx.send((chunk_index, result)).unwrap();
    }
}

#[allow(dead_code, reason = "alternative solution")]
fn compute_chunk_loop(x0: u64, y0: u64, chunk_index: usize) -> ChunkResult {
    let mut gen1 = Vec::with_capacity(CHUNK_SIZE / 4);
    let mut gen2 = Vec::with_capacity(CHUNK_SIZE / 8);
    let start = chunk_index * CHUNK_SIZE;
    let mut x = x0 * mod_pow(F1, start + 1) % MOD;
    let mut y = y0 * mod_pow(F2, start + 1) % MOD;
    let mut part1 = 0;
    for _ in 0..CHUNK_SIZE {
        part1 += usize::from((x as u16) == (y as u16));
        if start < N1 / 2 && x.is_multiple_of(4) {
            gen1.push(x as u16);
        }
        if y.is_multiple_of(8) {
            gen2.push(y as u16);
        }

        x = x * F1 % MOD;
        y = y * F2 % MOD;
    }

    ChunkResult { part1, gen1, gen2 }
}

fn compute_chunk_auto_vec(x0: u64, y0: u64, chunk_index: usize) -> ChunkResult {
    const VEC_SIZE: usize = 4;
    const VECS_PER_CHUNK: usize = CHUNK_SIZE.checked_div(VEC_SIZE).unwrap();
    const F1_TO_VEC_SIZE: u64 = mod_pow(F1, VEC_SIZE);
    const F2_TO_VEC_SIZE: u64 = mod_pow(F2, VEC_SIZE);

    let mut gen1 = Vec::with_capacity(CHUNK_SIZE / 4);
    let mut gen2 = Vec::with_capacity(CHUNK_SIZE / 8);
    let start = chunk_index * CHUNK_SIZE;
    let mut x: [_; VEC_SIZE] = array::from_fn(|i| x0 * mod_pow(F1, start + i + 1) % MOD);
    let mut y: [_; VEC_SIZE] = array::from_fn(|i| y0 * mod_pow(F2, start + i + 1) % MOD);
    let mut part1 = 0;
    for _ in 0..VECS_PER_CHUNK {
        // Encourage auto-vectorization and use instruction-level parallelism
        for i in 0..VEC_SIZE {
            part1 += usize::from((x[i] as u16) == (y[i] as u16));

            // Not producing unnecessary values here saves >30% runtime
            if start < N1 / 2 && x[i].is_multiple_of(4) {
                gen1.push(x[i] as u16);
            }
            if y[i].is_multiple_of(8) {
                gen2.push(y[i] as u16);
            }

            x[i] = x[i] * F1_TO_VEC_SIZE % MOD;
            y[i] = y[i] * F2_TO_VEC_SIZE % MOD;
        }
    }

    ChunkResult { part1, gen1, gen2 }
}

#[derive(Clone)]
struct ChunkResult {
    part1: usize,
    gen1: Vec<u16>,
    gen2: Vec<u16>,
}

fn consumer(x: u64, y: u64, rx: &Receiver<(usize, ChunkResult)>) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut consumed = 0;
    let mut chunks1 = FlattenedChunks::new();
    let mut chunks2 = FlattenedChunks::new();
    while let Ok((chunk_index, result)) = rx.recv() {
        part1 += result.part1;
        chunks1.chunks[chunk_index] = Some(result.gen1);
        chunks2.chunks[chunk_index] = Some(result.gen2);

        while let Some((slice1, slice2)) = chunks1.next().zip(chunks2.next()) {
            let limit = slice1.len().min(slice2.len()).min(N2 - consumed);
            for i in 0..limit {
                part2 += usize::from(slice1[i] == slice2[i]);
            }
            consumed += limit;
            chunks1.consume(limit);
            chunks2.consume(limit);
        }
    }

    part2 += chunks1
        .extend_to_infinite::<F1, 4>(x)
        .zip(chunks2.extend_to_infinite::<F2, 8>(y))
        .take(N2 - consumed)
        .filter(|(a, b)| a == b)
        .count();

    (part1, part2)
}

struct FlattenedChunks {
    chunks: [Option<Vec<u16>>; NUM_CHUNKS],
    index: usize,
    offset: usize,
}

impl FlattenedChunks {
    fn new() -> Self {
        Self {
            chunks: array::from_fn(|_| None),
            index: 0,
            offset: 0,
        }
    }

    fn next(&self) -> Option<&[u16]> {
        let opt = self.chunks.get(self.index)?;
        opt.as_deref().map(|chunk| &chunk[self.offset..])
    }

    fn consume(&mut self, n: usize) {
        self.offset += n;
        if self.offset == self.chunks[self.index].as_ref().unwrap().len() {
            self.index += 1;
            self.offset = 0;
        }
    }

    fn extend_to_infinite<const F: u64, const X: u64>(
        &self,
        x: u64,
    ) -> impl Iterator<Item = u16> + '_ {
        let lazy = generate::<F>(x, N1)
            .filter(|x| x.is_multiple_of(X))
            .map(|x| x as u16);
        self.chunks[self.index..]
            .iter()
            .enumerate()
            .filter_map(move |(i, chunk)| {
                let chunk = chunk.as_ref()?.as_slice();
                if i == 0 {
                    Some(&chunk[self.offset..])
                } else {
                    Some(chunk)
                }
            })
            .flatten()
            .copied()
            .chain(lazy)
    }
}

fn generate<const F: u64>(x: u64, n: usize) -> impl Iterator<Item = u64> {
    let x0 = x * mod_pow(F, n) % MOD;
    iter::successors(Some(x0), |x| Some(x * F % MOD))
}

const fn mod_pow(mut b: u64, mut e: usize) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 == 1 {
            r = (r * b) % MOD;
        }
        b = (b * b) % MOD;
        e >>= 1;
    }
    r
}

#[allow(dead_code, reason = "alternative solution")]
fn brute(x: u64, y: u64) -> (usize, usize) {
    let gen1 = generate::<F1>(x, 0);
    let gen2 = generate::<F2>(y, 0);
    let part1 = gen1
        .zip(gen2)
        .take(N1)
        .filter(|&(a, b)| a as u16 == b as u16)
        .count();

    let gen1 = generate::<F1>(x, 0).filter(|x| x % 4 == 0);
    let gen2 = generate::<F2>(y, 0).filter(|y| y % 8 == 0);
    let part2 = gen1
        .zip(gen2)
        .take(5_000_000)
        .filter(|(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
        .count();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("65 8921").unwrap(), (588, 309));
    }
}
