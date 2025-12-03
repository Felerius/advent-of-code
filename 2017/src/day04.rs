use register::register;
use utils::hash::{FastHashCollectionExt, FastHashSet};

#[register]
fn run(input: &str) -> (usize, usize) {
    let mut words = FastHashSet::with_capacity(16);
    let mut anagrams = FastHashSet::with_capacity(16);
    input
        .lines()
        .map(|line| {
            words.clear();
            anagrams.clear();
            line.split_ascii_whitespace()
                .map(|word| {
                    let anagram = word.bytes().fold([0; 26], |mut cnt, c| {
                        cnt[usize::from(c - b'a')] += 1;
                        cnt
                    });
                    (words.insert(word), anagrams.insert(anagram))
                })
                .fold((true, true), |(part1, part2), (p1, p2)| {
                    (part1 && p1, part2 && p2)
                })
        })
        .fold((0, 0), |(part1, part2), (p1, p2)| {
            (part1 + usize::from(p1), part2 + usize::from(p2))
        })
}
