pub fn run(input: &str) -> (i32, usize) {
    let (part1, part2) =
        input
            .bytes()
            .enumerate()
            .fold((0, None), |(mut floor, mut part2), (i, c)| {
                floor += if c == b'(' { 1 } else { -1 };
                if floor < 0 {
                    part2.get_or_insert(i + 1);
                }
                (floor, part2)
            });
    (part1, part2.unwrap_or(usize::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let inputs = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for (input, expected) in inputs {
            assert_eq!(run(input).0, expected, "failed for {input:?}");
        }
    }

    #[test]
    fn part2() {
        let inputs = [(")", 1), ("()())", 5)];
        for (input, expected) in inputs {
            assert_eq!(run(input).1, expected, "failed for {input:?}");
        }
    }
}
