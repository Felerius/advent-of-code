pub(crate) fn run(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| is_nice(line.as_bytes()))
        .fold((0, 0), |(part1, part2), (nice1, nice2)| {
            (part1 + usize::from(nice1), part2 + usize::from(nice2))
        })
}

const fn letter_bit(c: u8) -> u32 {
    1 << (c - b'a')
}

macro_rules! letter_set {
    ($($c:literal),*) => {{
        $(letter_bit($c) |)* 0
    }};
}

fn is_nice(s: &[u8]) -> (bool, bool) {
    const VOWELS: u32 = letter_set!(b'a', b'e', b'i', b'o', b'u');
    const FORBIDDEN_FIRST: u32 = letter_set!(b'a', b'c', b'p', b'x');
    let mut vowels = 0;
    let mut seen_pair = [[usize::MAX; 26]; 26];
    let mut double = false;
    let mut forbidden = false;
    let mut double_pair = false;
    let mut double_two_apart = false;
    for (i, &c2) in s.iter().enumerate() {
        vowels += usize::from((letter_bit(c2) & VOWELS) != 0);
        if let Some(c1) = i.checked_sub(1).map(|j| s[j]) {
            double |= c2 == c1;
            forbidden |= c1 + 1 == c2 && (letter_bit(c1) & FORBIDDEN_FIRST) != 0;

            let pair_entry = &mut seen_pair[usize::from(c1 - b'a')][usize::from(c2 - b'a')];
            double_pair |= i.saturating_sub(*pair_entry) > 1;
            if *pair_entry == usize::MAX {
                *pair_entry = i;
            }

            if let Some(c0) = i.checked_sub(2).map(|j| s[j]) {
                double_two_apart |= c0 == c2;
            }
        }
    }

    (
        vowels >= 3 && double && !forbidden,
        double_pair && double_two_apart,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let inputs = [
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];
        for (input, expected) in inputs {
            assert_eq!(
                is_nice(input.as_bytes()).0,
                expected,
                "failed for {input:?}"
            );
        }
    }

    #[test]
    fn part2() {
        let inputs = [
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];
        for (input, expected) in inputs {
            assert_eq!(
                is_nice(input.as_bytes()).1,
                expected,
                "failed for {input:?}"
            );
        }
    }
}
