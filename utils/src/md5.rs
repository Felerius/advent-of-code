use std::array;

pub type Digest = [u32; 4];

const INIT: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
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

pub fn prepare_for_len(buffer: &mut [u8; 64], len: usize) {
    debug_assert!(len < 56);
    buffer[len] = 0x80;
    buffer[56..].copy_from_slice(&(len * 8).to_le_bytes());
}

pub fn hash(buffer: &[u8; 64]) -> Digest {
    let [mut a, mut b, mut c, mut d] = INIT;
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
        a.wrapping_add(INIT[0]).to_be(),
        b.wrapping_add(INIT[1]).to_be(),
        c.wrapping_add(INIT[2]).to_be(),
        d.wrapping_add(INIT[3]).to_be(),
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

    #[test]
    fn hash() {
        let cases = [
            ("", 0xd41d8cd98f00b204e9800998ecf8427e),
            ("abc", 0x900150983cd24fb0d6963f7d28e17f72),
        ];
        for (input, expected) in cases {
            let mut buffer = [0; 64];
            prepare_for_len(&mut buffer, input.len());
            buffer[..input.len()].copy_from_slice(input.as_bytes());
            let [a, b, c, d] = super::hash(&buffer);
            let actual =
                u128::from(a) << 96 | u128::from(b) << 64 | u128::from(c) << 32 | u128::from(d);
            assert_eq!(
                expected, actual,
                "failed for {input:?}: [{a:08x}, {b:08x}, {c:08x}, {d:08x}]"
            );
        }
    }
}
