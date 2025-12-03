use anyhow::{Result, bail};

const MOD: usize = 100;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let mut pos = 50_usize;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        if let Some(num) = line.strip_prefix('L') {
            let num: usize = num.parse()?;
            let first_zero = if pos == 0 { MOD } else { pos };
            part2 += num.saturating_sub(first_zero - 1).div_ceil(MOD);
            pos = (pos + MOD - (num % MOD)) % MOD;
        } else if let Some(num) = line.strip_prefix('R') {
            let num: usize = num.parse()?;
            let first_zero = MOD - pos;
            part2 += num.saturating_sub(first_zero - 1).div_ceil(MOD);
            pos = (pos + num) % MOD;
        } else {
            bail!("invalid instruction: {line:?}");
        }

        part1 += usize::from(pos == 0);
    }

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (3, 6));
    }
}
