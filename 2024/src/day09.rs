use std::{array, cmp::Reverse, collections::BinaryHeap, iter};

use anyhow::{ensure, Result};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let input = input.as_bytes();
    ensure!(input.len() % 2 == 1);
    Ok((calc_part1(input), calc_part2(input)))
}

fn calc_part1(input: &[u8]) -> usize {
    let mut disk: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            let cnt = usize::from(b - b'0');
            let val = if i % 2 == 0 { i / 2 } else { usize::MAX };
            iter::repeat_n(val, cnt)
        })
        .collect();

    for i in 0.. {
        let Some(&val) = disk.get(i) else {
            break;
        };
        if val != usize::MAX {
            continue;
        }

        disk[i] = disk.pop().unwrap();
        while disk.last().is_some_and(|&v| v == usize::MAX) {
            disk.pop();
        }
    }

    disk.into_iter().enumerate().map(|(i, v)| i * v).sum()
}

fn calc_part2(input: &[u8]) -> usize {
    let num_files = input.len().div_ceil(2);
    let mut files = vec![(0, 0); num_files];
    let mut space_by_len: [_; 10] = array::from_fn(|_| BinaryHeap::new());
    let mut offset = 0;
    for (i, &c) in input.iter().enumerate() {
        let len = usize::from(c - b'0');
        if i % 2 == 0 {
            files[i / 2] = (len, offset);
        } else {
            space_by_len[len].push(Reverse(offset));
        }
        offset += len;
    }

    let mut checksum = ChecksumBuilder(0);
    for i in (0..num_files).rev() {
        let (file_len, file_offset) = files[i];
        let space = space_by_len
            .iter()
            .enumerate()
            .skip(file_len)
            .filter_map(|(len, heap)| heap.peek().map(|&Reverse(o)| (o, len)))
            .min()
            .filter(|&(o, _)| o < file_offset);
        if let Some((space_offset, space_len)) = space {
            checksum.add(space_offset, file_len, i);
            space_by_len[space_len].pop();
            space_by_len[space_len - file_len].push(Reverse(space_offset + file_len));
        } else {
            checksum.add(file_offset, file_len, i);
        }
    }

    checksum.0
}

struct ChecksumBuilder(usize);

impl ChecksumBuilder {
    fn add(&mut self, start: usize, len: usize, num: usize) {
        self.0 += (start..start + len).map(|i| num * i).sum::<usize>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("2333133121414131402").unwrap(), (1928, 2858));
    }
}
