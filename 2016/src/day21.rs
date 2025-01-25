use std::str::FromStr;

use anyhow::{bail, Error, Result};
use arrayvec::ArrayVec;
use itertools::Itertools;

const N: usize = 8;

pub(crate) fn run(input: &str) -> Result<(String, String)> {
    let instructions: Vec<_> = input.lines().map(Instruction::from_str).try_collect()?;

    let part1 = instructions
        .iter()
        .fold([0, 1, 2, 3, 4, 5, 6, 7], |word, &instr| instr.apply(word));
    let part2 = instructions
        .iter()
        .rev()
        .fold([5, 1, 6, 3, 2, 4, 0, 7], |word, &instr| instr.reverse(word));

    Ok((to_string(part1), to_string(part2)))
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SwapPositions(u8, u8),
    SwapLetters(u8, u8),
    Rotate(u8),
    RotateLetter(u8),
    ReverseRange(u8, u8),
    Move(u8, u8),
}

impl Instruction {
    fn apply(self, mut word: [u8; N]) -> [u8; N] {
        match self {
            Self::SwapPositions(x, y) => {
                word.swap(usize::from(x), usize::from(y));
            }
            Self::SwapLetters(x, y) => {
                for c in &mut word {
                    if *c == x {
                        *c = y;
                    } else if *c == y {
                        *c = x;
                    }
                }
            }
            Self::Rotate(x) => word.rotate_left(usize::from(x)),
            Self::RotateLetter(x) => {
                let a = word.iter().position(|&c| c == x).unwrap();
                word.rotate_right((a + 1 + usize::from(a >= 4)) % N);
            }
            Self::ReverseRange(x, y) => word[usize::from(x)..=usize::from(y)].reverse(),
            Self::Move(x, y) => {
                if x <= y {
                    word[usize::from(x)..=usize::from(y)].rotate_left(1);
                } else {
                    word[usize::from(y)..=usize::from(x)].rotate_right(1);
                }
            }
        }

        word
    }

    fn reverse(self, mut word: [u8; N]) -> [u8; N] {
        match self {
            Self::SwapPositions(x, y) => {
                word.swap(usize::from(x), usize::from(y));
            }
            Self::SwapLetters(x, y) => {
                for c in &mut word {
                    if *c == x {
                        *c = y;
                    } else if *c == y {
                        *c = x;
                    }
                }
            }
            Self::Rotate(x) => word.rotate_right(usize::from(x)),
            Self::RotateLetter(x) => {
                let a = word.iter().position(|&c| c == x).unwrap();
                let b = [7, 0, 4, 1, 5, 2, 6, 3][a];
                word.rotate_left((a + N - b) % N);
            }
            Self::ReverseRange(x, y) => word[usize::from(x)..=usize::from(y)].reverse(),
            Self::Move(x, y) => {
                if y <= x {
                    word[usize::from(y)..=usize::from(x)].rotate_left(1);
                } else {
                    word[usize::from(x)..=usize::from(y)].rotate_right(1);
                }
            }
        }

        word
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: ArrayVec<_, 7> = s.split_whitespace().collect();
        let instruction = if s.starts_with("swap position") {
            Self::SwapPositions(words[2].parse()?, words[5].parse()?)
        } else if s.starts_with("swap letter") {
            Self::SwapLetters(letter(words[2]), letter(words[5]))
        } else if s.starts_with("rotate left") {
            Self::Rotate(words[2].parse()?)
        } else if s.starts_with("rotate right") {
            Self::Rotate((8 - words[2].parse::<u8>()?) % 8)
        } else if s.starts_with("rotate based") {
            Self::RotateLetter(letter(words[6]))
        } else if s.starts_with("reverse positions") {
            Self::ReverseRange(words[2].parse()?, words[4].parse()?)
        } else if s.starts_with("move position") {
            Self::Move(words[2].parse()?, words[5].parse()?)
        } else {
            bail!("Unknown instruction: {s}");
        };
        Ok(instruction)
    }
}

fn to_string(word: [u8; N]) -> String {
    word.into_iter().map(|c| (c + b'a') as char).collect()
}

fn letter(s: &str) -> u8 {
    s.as_bytes()[0] - b'a'
}
