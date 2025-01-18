use anyhow::{Context, Result};
use num::{integer::ExtendedGcd, Integer};
use utils::input;

pub(crate) fn run(input: &str) -> Result<(i64, i64)> {
    let mut eq = (0, 1);
    let mut idx = 0;
    for line in input.lines() {
        idx += 1;
        let [_, m, _, x0] = input::integers(line);
        eq = combine_equations(eq, to_eq(x0, m, idx)).context("unsolvable")?;
    }
    let part1 = eq.0;
    eq = combine_equations(eq, to_eq(0, 11, idx + 1)).context("unsolvable")?;

    Ok((part1, eq.0))
}

fn to_eq(x0: i64, m: i64, i: usize) -> (i64, i64) {
    let x = (2 * m - x0 - i as i64 % m) % m;
    (x, m)
}

fn combine_equations((mut a, m): (i64, i64), (mut b, n): (i64, i64)) -> Option<(i64, i64)> {
    if n > m {
        return combine_equations((b, n), (a, m));
    }

    let ExtendedGcd { gcd, x, .. } = m.extended_gcd(&n);
    a %= m;
    b %= n;

    let l = m / gcd * n;
    let z = (b - a) % n * x % n / gcd * m + a;
    ((b - a) % gcd == 0).then(|| {
        let z = if z < 0 { z + l } else { z };
        (z, l)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "\
            Disc #1 has 5 positions; at time=0, it is at position 4.\n\
            Disc #2 has 2 positions; at time=0, it is at position 1.\n\
        ";
        assert_eq!(run(input).unwrap().0, 5);
    }
}
