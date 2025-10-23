#![allow(clippy::unreadable_literal, clippy::many_single_char_names)]
use std::{
    array,
    ops::{Deref, DerefMut},
};

pub type Digest = [u32; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleBlock([u8; 64]);

impl SingleBlock {
    #[must_use]
    pub fn new(len: usize) -> Self {
        assert!(len < 56);
        let mut buffer = [0; 64];
        buffer[len] = 0x80;
        write_len(buffer.last_chunk_mut().unwrap(), len);
        Self(buffer)
    }

    #[must_use]
    pub fn digest(&self) -> Digest {
        digest_to_be(hash_block(&self.0, INIT))
    }
}

impl Deref for SingleBlock {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SingleBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct Stack {
    bytes: Vec<u8>,
    prefix_digests: Vec<Digest>,
}

impl Stack {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
        if self.bytes.len() % 64 == 0 {
            let digest = self.prefix_digests.last().copied().unwrap_or(INIT);
            let digest = hash_block(self.bytes.last_chunk().unwrap(), digest);
            self.prefix_digests.push(digest);
        }
    }

    pub fn push_slice(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.push(byte);
        }
    }

    pub fn pop(&mut self) {
        if self.bytes.len() % 64 == 0 {
            self.prefix_digests.pop();
        }
        self.bytes.pop();
    }

    pub fn digest(&mut self) -> Digest {
        let original_len = self.bytes.len();
        self.bytes.push(0x80);
        self.bytes
            .resize((self.bytes.len() + 8).next_multiple_of(64), 0);
        write_len(self.bytes.last_chunk_mut().unwrap(), original_len);

        let mut digest = self.prefix_digests.last().copied().unwrap_or(INIT);
        let start_block = self.prefix_digests.len() * 64;
        digest = hash_block(self.bytes[start_block..].first_chunk().unwrap(), digest);
        if self.bytes.len() > start_block + 64 {
            digest = hash_block(
                self.bytes[start_block + 64..].first_chunk().unwrap(),
                digest,
            );
        }

        self.bytes.truncate(original_len);
        digest_to_be(digest)
    }
}

const INIT: Digest = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];
const I: [usize; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1, 6, 11, 0, 5, 10, 15, 4, 9, 14, 3, 8,
    13, 2, 7, 12, 5, 8, 11, 14, 1, 4, 7, 10, 13, 0, 3, 6, 9, 12, 15, 2, 0, 7, 14, 5, 12, 3, 10, 1,
    8, 15, 6, 13, 4, 11, 2, 9,
];

macro_rules! round4 {
    ($func:ident, $off:expr, $a:ident, $b:ident, $c:ident, $d:ident, $m:ident, $i:ident, $s:ident, $k:ident) => {
        $a = $func($a, $b, $c, $d, $m[$i[$off + 0]], $s[$off + 0], $k[$off + 0]);
        $d = $func($d, $a, $b, $c, $m[$i[$off + 1]], $s[$off + 1], $k[$off + 1]);
        $c = $func($c, $d, $a, $b, $m[$i[$off + 2]], $s[$off + 2], $k[$off + 2]);
        $b = $func($b, $c, $d, $a, $m[$i[$off + 3]], $s[$off + 3], $k[$off + 3]);
    };
}

macro_rules! round16 {
    ($func:ident, $off:literal, $a:ident, $b:ident, $c:ident, $d:ident, $m:ident, $i:ident, $s:ident, $k:ident) => {
        round4!($func, $off + 0, $a, $b, $c, $d, $m, $i, $s, $k);
        round4!($func, $off + 4, $a, $b, $c, $d, $m, $i, $s, $k);
        round4!($func, $off + 8, $a, $b, $c, $d, $m, $i, $s, $k);
        round4!($func, $off + 12, $a, $b, $c, $d, $m, $i, $s, $k);
    };
}

fn digest_to_be(digest: Digest) -> Digest {
    digest.map(u32::to_be)
}

fn write_len(bytes: &mut [u8; 8], num_bytes: usize) {
    *bytes = (num_bytes * 8).to_le_bytes();
}

fn hash_block(buffer: &[u8; 64], digest: Digest) -> Digest {
    let [mut a, mut b, mut c, mut d] = digest;
    let m: [_; 16] = array::from_fn(|i| {
        u32::from_le_bytes([
            buffer[i * 4],
            buffer[i * 4 + 1],
            buffer[i * 4 + 2],
            buffer[i * 4 + 3],
        ])
    });

    round16!(round1, 0, a, b, c, d, m, I, S, K);
    round16!(round2, 16, a, b, c, d, m, I, S, K);
    round16!(round3, 32, a, b, c, d, m, I, S, K);
    round16!(round4, 48, a, b, c, d, m, I, S, K);

    [
        a.wrapping_add(digest[0]),
        b.wrapping_add(digest[1]),
        c.wrapping_add(digest[2]),
        d.wrapping_add(digest[3]),
    ]
}

fn round1(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & c) | (!b & d);
    common(f, a, b, m, s, k)
}

fn round2(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & d) | (c & !d);
    common(f, a, b, m, s, k)
}

fn round3(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = b ^ c ^ d;
    common(f, a, b, m, s, k)
}

fn round4(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = c ^ (b | !d);
    common(f, a, b, m, s, k)
}

fn common(f: u32, a: u32, b: u32, m: u32, s: u32, k: u32) -> u32 {
    f.wrapping_add(a)
        .wrapping_add(k)
        .wrapping_add(m)
        .rotate_left(s)
        .wrapping_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_u128(digest: Digest) -> u128 {
        u128::from(digest[0]) << 96
            | u128::from(digest[1]) << 64
            | u128::from(digest[2]) << 32
            | u128::from(digest[3])
    }

    #[test]
    fn single_block() {
        let cases = [
            ("", 0xd41d8cd98f00b204e9800998ecf8427e),
            ("abc", 0x900150983cd24fb0d6963f7d28e17f72),
        ];
        for (input, expected) in cases {
            let mut block = SingleBlock::new(input.len());
            block[..input.len()].copy_from_slice(input.as_bytes());
            let actual = block.digest();
            assert_eq!(expected, to_u128(actual), "failed for {input:?}");
        }
    }

    #[test]
    fn stack() {
        const A64: &[u8] = &[b'a'; 64];

        let mut stack = Stack::new();
        assert_eq!(0xd41d8cd98f00b204e9800998ecf8427e, to_u128(stack.digest()));

        stack.push_slice(A64);
        assert_eq!(0x014842d480b571495a4a0363793f7367, to_u128(stack.digest()));

        stack.pop();
        assert_eq!(0xb06521f39153d618550606be297466d5, to_u128(stack.digest()));

        stack.push(b'a');
        assert_eq!(0x014842d480b571495a4a0363793f7367, to_u128(stack.digest()));

        stack.push(b'a');
        assert_eq!(0xc743a45e0d2e6a95cb859adae0248435, to_u128(stack.digest()));

        stack.push_slice(A64);
        assert_eq!(0xb325dc1c6f5e7a2b7cf465b9feab7948, to_u128(stack.digest()));
    }
}
