use anyhow::Result;
use itertools::Itertools;
use register::register;

#[register]
fn run(input: &str) -> Result<(usize, u64)> {
    let mut events: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| -> Result<_> {
            if let Some((start, end)) = l.split_once('-') {
                let start: u64 = start.parse()?;
                let end: u64 = end.parse()?;
                Ok([
                    Some((start, EventType::StartRange)),
                    Some((end + 1, EventType::EndRange)),
                ])
            } else {
                let num: u64 = l.parse()?;
                Ok([Some((num, EventType::Query)), None])
            }
        })
        .flatten_ok()
        .flatten_ok()
        .try_collect()?;
    events.sort_unstable();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut num_active = 0;
    let mut prev = None;
    for (pos, event_type) in events {
        if let Some(prev) = prev
            && num_active > 0
        {
            part2 += pos - prev;
        }
        prev = Some(pos);

        match event_type {
            EventType::StartRange => {
                num_active += 1;
            }
            EventType::EndRange => {
                num_active -= 1;
            }
            EventType::Query => {
                part1 += usize::from(num_active > 0);
            }
        }
    }

    Ok((part1, part2))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    EndRange,
    StartRange,
    Query,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (3, 14));
    }
}
