use itertools::{EitherOrBoth, Itertools};
use utils::input;

pub(crate) fn run(input: &str) -> (u64, u64) {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let [a, b] = input::integers::<u32, 2>(line);
            (a, b)
        })
        .unzip();
    list1.sort_unstable();
    list2.sort_unstable();

    let part1 = list1
        .iter()
        .zip(&list2)
        .map(|(&a, &b)| u64::from(a.abs_diff(b)))
        .sum();

    let chunks1 = list1
        .chunk_by(|a, b| a == b)
        .map(|chunk| (chunk[0], chunk.len()));
    let chunks2 = list2
        .chunk_by(|a, b| a == b)
        .map(|chunk| (chunk[0], chunk.len()));
    let part2: u64 = chunks1
        .merge_join_by(chunks2, |(a, _), (b, _)| a.cmp(b))
        .map(|entry| {
            if let EitherOrBoth::Both((x, cnt_l), (_, cnt_r)) = entry {
                u64::from(x) * cnt_l as u64 * cnt_r as u64
            } else {
                0
            }
        })
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        assert_eq!(run(input), (11, 31));
    }
}
