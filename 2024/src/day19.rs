use std::collections::VecDeque;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tinybitset::TinyBitSet;

const ALPHABET: usize = 5;
const MAX_PATTERN_LEN: usize = 8;

pub fn run(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let mut aho_corasick = AhoCorasick::new();
    for pattern in lines.next().unwrap().split(", ") {
        aho_corasick.add_pattern(pattern.bytes().map(color_from_char));
    }
    aho_corasick.finalize();

    let haystacks: Vec<_> = lines.skip(1).collect();
    haystacks
        .into_par_iter()
        .map(|line| {
            let mut dp = [0; MAX_PATTERN_LEN];
            let mut node = 0;
            dp[0] = 1;
            for (i, color) in line.bytes().map(color_from_char).enumerate() {
                node = aho_corasick.trie[node][color];
                dp[(i + 1) % MAX_PATTERN_LEN] = aho_corasick.matching[node]
                    .iter()
                    .map(|j| dp[(i - j) % MAX_PATTERN_LEN])
                    .sum();
            }

            let cnt = dp[line.len() % MAX_PATTERN_LEN];
            (usize::from(cnt > 0), cnt)
        })
        .reduce(|| (0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
}

struct AhoCorasick {
    trie: Vec<[usize; ALPHABET]>,
    matching: Vec<TinyBitSet<u8, 1>>,
}

impl AhoCorasick {
    fn new() -> Self {
        Self {
            trie: vec![[0; ALPHABET]],
            matching: vec![TinyBitSet::new()],
        }
    }

    fn add_pattern(&mut self, pattern: impl IntoIterator<Item = usize>) {
        let mut node = 0;
        let mut len = 0;
        for color in pattern {
            if self.trie[node][color] == 0 {
                self.trie[node][color] = self.trie.len();
                self.trie.push([0; 5]);
                self.matching.push(TinyBitSet::new());
            }
            node = self.trie[node][color];
            len += 1;
        }
        self.matching[node].insert(len - 1);
    }

    fn finalize(&mut self) {
        let mut queue = VecDeque::from([(0, 0)]);
        while let Some((node, link)) = queue.pop_front() {
            for color in 0..ALPHABET {
                if self.trie[node][color] == 0 {
                    self.trie[node][color] = self.trie[link][color];
                } else {
                    let next = self.trie[node][color];
                    let next_link = if node == 0 { 0 } else { self.trie[link][color] };
                    let link_matching = self.matching[next_link];
                    self.matching[next] |= link_matching;
                    queue.push_back((next, next_link));
                }
            }
        }
    }
}

fn color_from_char(c: u8) -> usize {
    match c {
        b'r' => 0,
        b'g' => 1,
        b'b' => 2,
        b'u' => 3,
        b'w' => 4,
        _ => panic!("Invalid color: {:?}", char::from(c)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (6, 16));
    }
}
