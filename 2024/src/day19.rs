use anyhow::Result;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut lines = input.lines();
    let patterns: Vec<_> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|p| p.as_bytes())
        .collect();

    let (part1, part2) = lines
        .skip(1)
        .map(|line| {
            let line = line.as_bytes();
            let mut dp = vec![0; line.len() + 1];
            dp[0] = 1;
            for i in 1..=line.len() {
                dp[i] = patterns
                    .iter()
                    .map(|&pattern| {
                        let matches = pattern.len() <= i && &line[i - pattern.len()..i] == pattern;
                        if matches {
                            dp[i - pattern.len()]
                        } else {
                            0
                        }
                    })
                    .sum();
            }

            dp[line.len()]
        })
        .fold((0, 0), |(part1, part2), count| {
            (part1 + usize::from(count > 0), part2 + count)
        });

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (6, 16));
    }
}
