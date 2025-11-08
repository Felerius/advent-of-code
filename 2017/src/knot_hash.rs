use std::{iter, ops::BitXor};

pub(crate) fn hash_lengths(lengths: impl IntoIterator<Item = u8>) -> [u8; 256] {
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

pub(crate) fn hash_str(s: &str) -> u128 {
    let lengths = s.bytes().chain([17, 31, 73, 47, 23]);
    let lengths = iter::repeat_n(lengths, 64).flatten();
    let bytes = hash_lengths(lengths);
    let (chunks, _) = bytes.as_chunks::<16>();
    let chunks_array: &[_; 16] = chunks.try_into().expect("size known at compile time");
    let chunk_bytes = chunks_array.map(|chunk| chunk.into_iter().fold(0, BitXor::bitxor));
    u128::from_be_bytes(chunk_bytes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_str() {
        let cases = [
            ("", 0xa258_2a3a_0e66_e6e8_6e38_12dc_b672_a272),
            ("AoC 2017", 0x33ef_eb34_ea91_902b_b2f5_9c99_20ca_a6cd),
            ("1,2,3", 0x3efb_e78a_8d82_f299_7903_1a4a_a0b1_6a9d),
            ("1,2,4", 0x6396_0835_bcdc_130f_0b66_d7ff_4f6a_5a8e),
        ];
        for (input, expected) in cases {
            let actual = super::hash_str(input);
            assert_eq!(
                actual, expected,
                "failed for input {input:?} ({actual:032x} vs. {expected:032x})"
            );
        }
    }
}
