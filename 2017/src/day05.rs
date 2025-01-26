use std::iter;

use anyhow::Result;
use itertools::Itertools;

const BLOCK_SIZE: usize = 16;
type BlockBitset = u16;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let jumps: Vec<_> = input.lines().map(str::parse).try_collect()?;
    Ok((part1(jumps.clone()), part2(jumps)))
}

fn part1(mut jumps: Vec<isize>) -> usize {
    iter::successors(Some(0), |&i| {
        let j = usize::try_from(i).ok()?;
        let jmp = jumps.get_mut(j)?;
        let i = i + *jmp;
        *jmp += 1;
        Some(i)
    })
    .skip(1)
    .count()
}

fn is_ready(jmp: isize) -> bool {
    jmp == 2 || jmp == 3
}

fn make_block(jumps: &[isize]) -> BlockBitset {
    jumps
        .iter()
        .rev()
        .fold(0, |bs, &jmp| (bs << 1) | BlockBitset::from(jmp == 3))
}

fn calculate_block(bs: BlockBitset, i0: usize) -> (i8, u8, BlockBitset) {
    let mut i = i0 as isize;
    let mut steps = 0;
    let mut cur_bs = bs as BlockBitset;
    while (0..BLOCK_SIZE as isize).contains(&i) {
        steps += 1;
        let jmp = 2 + ((cur_bs >> i) & 1) as isize;
        cur_bs ^= 1 << i;
        i += jmp;
    }

    ((i - i0 as isize) as i8, steps as u8, cur_bs)
}

fn part2(mut jumps: Vec<isize>) -> usize {
    // let block_precalc: Vec<[(isize, usize, BlockBitset); BLOCK_SIZE]> = (0..1 << BLOCK_SIZE)
    //     .map(|bs| {
    //         array::from_fn(|i0| {
    //             let mut i = i0 as isize;
    //             let mut steps = 0;
    //             let mut cur_bs = bs as BlockBitset;
    //             while (0..BLOCK_SIZE as isize).contains(&i) {
    //                 steps += 1;
    //                 let jmp = 2 + ((cur_bs >> i) & 1) as isize;
    //                 cur_bs ^= 1 << i;
    //                 i += jmp;
    //             }

    //             (i - i0 as isize, steps, cur_bs)
    //         })
    //     })
    //     .collect();

    let mut block_cache = vec![[(0, 0, BlockBitset::default()); BLOCK_SIZE]; 1 << BLOCK_SIZE];
    let mut block_ready: Vec<_> = jumps
        .chunks(BLOCK_SIZE)
        .map(|block| block.iter().filter(|&&jmp| is_ready(jmp)).count() as u8)
        .collect();
    let mut blocks: Vec<_> = jumps.chunks(BLOCK_SIZE).map(make_block).collect();

    let mut i = 0;
    let mut steps = 0;
    loop {
        let block = i / BLOCK_SIZE;
        let jmp = if usize::from(block_ready[block]) == BLOCK_SIZE {
            let cached = &mut block_cache[usize::from(blocks[block])][i % BLOCK_SIZE];
            if cached.1 == 0 {
                *cached = calculate_block(blocks[block], i % BLOCK_SIZE);
            }
            let (jmp, block_steps, new_block) = *cached;

            steps += usize::from(block_steps);
            blocks[block] = new_block;
            isize::from(jmp)
        } else {
            steps += 1;
            let jmp = jumps[i];
            let ready_before = jmp == 2 || jmp == 3;

            jumps[i] = if jumps[i] >= 3 {
                jumps[i] - 1
            } else {
                jumps[i] + 1
            };

            let ready_after = jumps[i] == 2 || jumps[i] == 3;
            block_ready[block] += u8::from(!ready_before && ready_after);
            if usize::from(block_ready[block]) == BLOCK_SIZE {
                blocks[block] = make_block(&jumps[block * BLOCK_SIZE..(block + 1) * BLOCK_SIZE]);
            }

            jmp
        };

        let Some(i2) = i.checked_add_signed(jmp).filter(|&i| i < jumps.len()) else {
            break steps;
        };
        i = i2;
    }
}
