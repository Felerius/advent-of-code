use anyhow::Result;
use itertools::Itertools;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let (part1, part2) = input.trim().lines().fold((0, 0), |(part1, part2), line| {
        let nums: Vec<_> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let safe1 = is_safe(&nums);
        let safe2 = safe1
            || (0..nums.len()).any(|i| {
                let without = nums
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, n)| n);
                is_safe(without)
            });
        (part1 + usize::from(safe1), part2 + usize::from(safe2))
    });

    Ok((part1, part2))
}

fn is_safe<'a>(iter: impl IntoIterator<Item = &'a u32>) -> bool {
    let mut iter = iter.into_iter().copied();
    let Some((item0, item1)) = iter.next_tuple() else {
        return true;
    };
    let iter = [item0, item1].into_iter().chain(iter);
    if item0 < item1 {
        iter.is_sorted_by(|a, b| a < b && b - a <= 3)
    } else {
        iter.is_sorted_by(|a, b| a > b && a - b <= 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ";
        assert_eq!(run(input).unwrap(), (2, 4));
    }
}
