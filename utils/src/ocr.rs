use anyhow::{Context, Result};

const CHARACTERS: [(u32, u8); 20] = [
    (0x19297a52, b'A'),
    (0x392e4a5c, b'B'),
    (0x1928424c, b'C'),
    (0x39294a5c, b'D'),
    (0x3d0e421e, b'E'),
    (0x3d0e4210, b'F'),
    (0x19285a4e, b'G'),
    (0x252f4a52, b'H'),
    (0x1c42108e, b'I'),
    (0x0c210a4c, b'J'),
    (0x254c5292, b'K'),
    (0x2108421e, b'L'),
    (0x19294a4c, b'O'),
    (0x39297210, b'P'),
    (0x39297292, b'R'),
    (0x1d08305c, b'S'),
    (0x1c421084, b'T'),
    (0x25294a4c, b'U'),
    (0x23151084, b'Y'),
    (0x3c22221e, b'Z'),
];

pub fn character(bits: u32) -> Result<u8> {
    CHARACTERS
        .into_iter()
        .find_map(|(c, ch)| (c == bits).then_some(ch))
        .context("character not recognized")
}
