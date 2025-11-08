use std::array;

use anyhow::{Context, Result, bail};

pub(crate) fn run(input: &str) -> Result<(String, String)> {
    run_var_length::<16>(input)
}

fn run_var_length<const N: usize>(input: &str) -> Result<(String, String)> {
    // Insights:
    //
    // - Instructions modifying positions (spin, exchange) are independent from
    //   those modifying labels (partner). If a partner instruction swaps two
    //   positions based on their labels, this doesn't have any effect on later
    //   spin/exchange instructions, as they don't care about the labels at the
    //   positions they swap. Thus, we can reorder instructions arbitrarily, as
    //   long as we keep the relative order of spin & exchange instructions, and
    //   the relative order of partner instructions.
    // - Both instruction types can be combined into permutations: for spin &
    //   exchange, this is the usual permutation of positions that describes
    //   where each position ends up; for partner, this is the same but for
    //   labels. These permutations can then be exponentiated cheaply to apply
    //   them the required 1 billion times.
    //
    // Overall, we reformulate the sequence of instructions as follows:
    //
    // - (mixed_instructions)^1B
    // - = (position_instructions * label_instructions)^1B
    // - = (position_instructions)^1B * (label_instructions)^1B
    let mut pos = Permutation::<N>::identity();
    let mut label = Permutation::<N>::identity();
    for instr in input.split(',') {
        if let Some(tail) = instr.strip_prefix('s') {
            pos.0.rotate_right(tail.parse()?);
        } else if let Some(tail) = instr.strip_prefix('x') {
            let (a, b) = tail.split_once('/').context("unknown instruction")?;
            pos.0.swap(a.parse()?, b.parse()?);
        } else if let Some(tail) = instr.strip_prefix('p') {
            let tail = tail.as_bytes();
            let a = usize::from(tail[0] - b'a');
            let b = usize::from(tail[2] - b'a');
            label.0.swap(a, b);
        } else {
            bail!("unknown instruction: {instr}");
        }
    }

    let label_perm = label.invert();
    let part1 = pos.stringify(label_perm);
    let part2 = pos
        .invert()
        .pow(1_000_000_000)
        .invert()
        .stringify(label_perm.pow(1_000_000_000));

    Ok((part1, part2))
}

#[derive(Debug, Clone, Copy)]
struct Permutation<const N: usize>([u8; N]);

impl<const N: usize> Permutation<N> {
    fn identity() -> Self {
        Self(array::from_fn(|i| i as u8))
    }

    fn invert(self) -> Self {
        let p = self
            .0
            .into_iter()
            .enumerate()
            .fold([0u8; N], |mut inv, (i, v)| {
                inv[usize::from(v)] = i as u8;
                inv
            });
        Self(p)
    }

    fn mul(self, other: Self) -> Self {
        Self(array::from_fn(|i| other.0[usize::from(self.0[i])]))
    }

    fn pow(self, mut exp: u64) -> Self {
        let mut base = self;
        let mut result = Self::identity();
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(base);
            }
            base = base.mul(base);
            exp /= 2;
        }
        result
    }

    fn stringify(self, label_perm: Self) -> String {
        let bytes = self.0.map(|x| b'a' + label_perm.0[usize::from(x)]);
        str::from_utf8(&bytes).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (part1, _) = run_var_length::<5>("s1,x3/4,pe/b").unwrap();
        assert_eq!(part1, "baedc");
    }
}
