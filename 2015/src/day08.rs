pub fn run(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| {
            let line = &line.as_bytes()[1..line.len() - 1];
            let mut ans1 = 2;
            let mut ans2 = 4;
            let mut i = 0;
            while let Some(&c) = line.get(i) {
                match c {
                    b'\\' if line.get(i + 1).is_some_and(|&c2| c2 == b'\\' || c2 == b'"') => {
                        ans1 += 1;
                        ans2 += 2;
                        i += 2;
                    }
                    b'\\' if line.get(i + 1) == Some(&b'x') => {
                        ans1 += 3;
                        ans2 += 1;
                        i += 4;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }

            (ans1, ans2)
        })
        .fold((0, 0), |(part1, part2), (p1, p2)| (part1 + p1, part2 + p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let inputs = [
            ("\"\"", 2),
            ("\"abc\"", 2),
            ("\"aaa\\\"aaa\"", 3),
            ("\"\\x27\"", 5),
        ];
        for (input, expected) in inputs {
            assert_eq!(run(input).0, expected, "failed for {input:?}");
        }
    }

    #[test]
    fn part2() {
        let inputs = [];
        for (input, expected) in inputs {
            assert_eq!(run(input).1, expected, "failed for {input:?}");
        }
    }
}
