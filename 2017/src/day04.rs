use utils::hash::{FastHashCollectionExt, FastHashSet};

pub(crate) fn run(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| {
            let mut words = FastHashSet::new();
            let mut anagrams = FastHashSet::new();
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
