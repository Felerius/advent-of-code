use anyhow::{ensure, Context, Result};
use arrayvec::{ArrayString, ArrayVec};

const LEN: usize = 8;

pub fn run(input: &str) -> Result<(ArrayString<LEN>, ArrayString<LEN>)> {
    let mut lines = input.lines().peekable();
    let line_len = lines.peek().context("empty input")?.len();
    ensure!(line_len <= LEN, "line length too long");

    let counts = ArrayVec::<_, LEN>::from_iter((0..line_len).map(|_| [0; 26]));
    let counts = lines.fold(counts, |mut counts, line| {
        for (i, c) in line.bytes().enumerate() {
            counts[i][usize::from(c - b'a')] += 1;
        }
        counts
    });

    let mut part1 = ArrayString::new();
    let mut part2 = ArrayString::new();
    for cnt in counts {
        let c1 = (0..26).max_by_key(|&i| cnt[usize::from(i)]).unwrap();
        let c2 = (0..26)
            .filter(|&i| cnt[usize::from(i)] != 0)
            .min_by_key(|&i| cnt[usize::from(i)])
            .unwrap();
        part1.push(char::from(b'a' + c1));
        part2.push(char::from(b'a' + c2));
    }
    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

    #[test]
    fn test() {
        let (part1, part2) = run(INPUT).unwrap();
        assert_eq!("easter", &part1);
        assert_eq!("advent", &part2);
    }
}
