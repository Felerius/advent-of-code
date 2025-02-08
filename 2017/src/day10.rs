use std::{fmt::Write, iter, ops::BitXor};

use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run(input: &str) -> Result<(u16, String)> {
    let part1_lengths: Vec<_> = input.split(',').map(str::parse).try_collect()?;
    let part1_hash = knot_hash(part1_lengths);
    let part1 = u16::from(part1_hash[0]) * u16::from(part1_hash[1]);
    let part2 = knot_hash_str(input.trim());

    Ok((part1, part2))
}

fn knot_hash(lengths: impl IntoIterator<Item = u8>) -> [u8; 256] {
    let mut nums = [0; 256];
    for i in 0..=255 {
        nums[usize::from(i)] = i;
    }

    let mut offset = 0;
    for (i, num) in lengths.into_iter().map(usize::from).enumerate() {
        nums[..num].reverse();
        offset += i + num;
        nums.rotate_left((i + num) % 256);
    }

    nums.rotate_right(offset % 256);
    nums
}

fn knot_hash_str(s: &str) -> String {
    let lengths = s.bytes().chain([17, 31, 73, 47, 23]);
    let lengths = iter::repeat_n(lengths, 64).flatten();
    knot_hash(lengths)
        .chunks_exact(16)
        .map(|chunk| chunk.iter().copied().fold(0, BitXor::bitxor))
        .fold(String::with_capacity(32), |mut s, n| {
            write!(s, "{n:02x}").expect("writing to string failed");
            s
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn knot_hash_str() {
        let cases = [
            ("", "a2582a3a0e66e6e86e3812dcb672a272"),
            ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"),
            ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"),
            ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"),
        ];
        for (input, expected) in cases {
            let actual = super::knot_hash_str(input);
            assert_eq!(actual, expected, "failed for input: {input:?}");
        }
    }
}
