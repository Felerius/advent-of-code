use std::iter;

use anyhow::{Context, Result};

pub fn run(input: &str) -> Result<(u32, u32)> {
    let mut adj = [[0; 8]; 8];
    let mut cities = [""; 8];
    let mut num_cities = 0;
    for line in input.lines() {
        let (head, tail) = line.split_once(" = ").context("invalid input")?;
        let (city1, city2) = head.split_once(" to ").context("invalid input")?;
        let city1 = cities.iter().position(|&c| c == city1).unwrap_or_else(|| {
            cities[num_cities] = city1;
            num_cities += 1;
            num_cities - 1
        });
        let city2 = cities.iter().position(|&c| c == city2).unwrap_or_else(|| {
            cities[num_cities] = city2;
            num_cities += 1;
            num_cities - 1
        });
        adj[city1][city2] = tail.parse().context("invalid input")?;
        adj[city2][city1] = adj[city1][city2];
    }

    let mut dp = [[(u32::MAX, 0_u32); 8]; 1 << 8];
    for c in 0..num_cities {
        dp[1 << c][c] = (0, 0);
    }

    for m in 1_usize..(1 << num_cities) {
        if m.count_ones() == 1 {
            continue;
        }

        for c1 in bits(m) {
            let m2 = m ^ (1 << c1);
            for c2 in bits(m2) {
                dp[m][c1].0 = dp[m][c1].0.min(dp[m2][c2].0.saturating_add(adj[c2][c1]));
                dp[m][c1].1 = dp[m][c1].1.max(dp[m2][c2].1.saturating_add(adj[c2][c1]));
            }
        }
    }

    let solution = dp[(1 << num_cities) - 1]
        .into_iter()
        .fold((u32::MAX, 0), |(part1, part2), (mn, mx)| {
            (part1.min(mn), part2.max(mx))
        });
    Ok(solution)
}

fn bits(mut m: usize) -> impl Iterator<Item = usize> {
    iter::from_fn(move || {
        (m != 0).then(|| {
            let c = m.trailing_zeros() as usize;
            m ^= 1 << c;
            c
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).unwrap().0, 605);
    }

    #[test]
    fn part2() {
        assert_eq!(run(INPUT).unwrap().1, 982);
    }
}
