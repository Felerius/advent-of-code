use register::register;

#[register]
fn run(input: &str) -> (u64, u64) {
    run_testable(input, u64::from(u32::MAX))
}

fn run_testable(input: &str, max: u64) -> (u64, u64) {
    let mut events: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let (start, end) = line.split_once('-').expect("invalid input line");
            let start: u64 = start.parse().expect("invalid input line");
            let end: u64 = end.parse().expect("invalid input line");
            [(start, EventType::Start), (end + 1, EventType::End)]
        })
        .collect();
    events.sort_unstable();

    let (next, _, mut part1, mut part2) = events.into_iter().fold(
        (0, 0, None, 0),
        |(next, cnt, mut part1, mut part2), (x, ty)| {
            if cnt == 0 && x > next {
                part1 = part1.or(Some(next));
                part2 += x - next;
            }
            let cnt = match ty {
                EventType::Start => cnt + 1,
                EventType::End => cnt - 1,
            };
            (x, cnt, part1, part2)
        },
    );

    if max >= next {
        part1 = part1.or(Some(next));
        part2 += max - next + 1;
    }

    (part1.expect("no unblocked addresses"), part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    End,
    Start,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "5-8\n0-2\n4-7";
        assert_eq!(run_testable(input, 9), (3, 2));
    }
}
