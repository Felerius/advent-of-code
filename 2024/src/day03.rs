pub fn run(input: &str) -> (u32, u32) {
    let (part1, part2, _) = itertools::kmerge(
        ["mul(", "do()", "don't()"].map(|s| input.match_indices(s)),
    )
    .fold((0, 0, true), |(part1, part2, enabled), (i, mat)| {
        if mat == "mul(" {
            let x = try_parse(&input[i..]).unwrap_or(0);
            (part1 + x, part2 + u32::from(enabled) * x, enabled)
        } else if mat == "do()" {
            (part1, part2, true)
        } else {
            (part1, part2, false)
        }
    });

    (part1, part2)
}

fn try_parse(s: &str) -> Option<u32> {
    let end = s[4..s.len().min(12)].find(')')?;
    let (a, b) = s[4..4 + end].split_once(',')?;
    let a: u32 = a.parse().ok()?;
    let b: u32 = b.parse().ok()?;
    Some(a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(run(input).0, 161);
    }

    #[test]
    fn part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(run(input).1, 48);
    }
}
