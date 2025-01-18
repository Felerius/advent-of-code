use arrayvec::ArrayString;

const LEN: usize = 8;

pub(crate) fn run(input: &str) -> (ArrayString<LEN>, ArrayString<LEN>) {
    let input = to_int(input);
    let part1 = next(input);
    let part2 = next(part1 + 1);
    (to_string(part1), to_string(part2))
}

fn to_int(s: &str) -> u64 {
    s.bytes().fold(0, |i, c| i * 26 + u64::from(c - b'a'))
}

fn to_string(mut i: u64) -> ArrayString<LEN> {
    let mut bytes = [0; LEN];
    for j in 0..LEN {
        bytes[LEN - 1 - j] = (i % 26) as u8 + b'a';
        i /= 26;
    }
    ArrayString::from_byte_string(&bytes).expect("bug in int to string conversion")
}

fn next(i: u64) -> u64 {
    // This assumes that the 3 character prefix do not contribute to the two
    // pairs or the increasing triple. If this is the case, the 5 character
    // suffix has to be of the form "aabcc", "bbcdd", etc.

    /// Encoded form of "aabcc"
    const BASE_SUFFIX: u64 = (26 + 2) * 26 + 2;
    /// Increment from from "aabcc" to "bbcdd" (and so on)
    const INCREMENT: u64 = (26_u64.pow(5) - 1) / 25;
    /// All suffixes that don't contain any of the forbidden characters
    const VALID_SUFFIXES: [u64; 15] = [0, 1, 2, 3, 4, 5, 15, 16, 17, 18, 19, 20, 21, 22, 23];
    const PREFIX_STEP: u64 = 26_u64.pow(5);

    let i_prefix = i / PREFIX_STEP * PREFIX_STEP;
    VALID_SUFFIXES
        .into_iter()
        .map(|j| {
            let suffix = BASE_SUFFIX + j * INCREMENT;
            if i_prefix + suffix >= i {
                i_prefix + suffix
            } else {
                i_prefix + PREFIX_STEP + suffix
            }
        })
        .min()
        .expect("bug in suffix enumeration")
}
