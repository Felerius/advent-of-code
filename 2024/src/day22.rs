use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use register::register;

const NUM_SEQ: usize = 19_usize.pow(4);

#[register]
fn run(input: &str) -> (u64, u32) {
    let lines: Vec<_> = input.lines().collect();
    let (part1, gain) = lines
        .into_par_iter()
        .enumerate()
        .fold(
            || (0, vec![u16::MAX; NUM_SEQ], vec![0_u32; NUM_SEQ]),
            |(part1, mut seen, mut gain), (i, line)| {
                let mut num = line.parse().unwrap();
                let mut seq = 0;
                for j in 0..2000 {
                    let next_num = next(num);
                    let diff = next_num % 10 + 9 - num % 10;
                    seq = seq % 19_usize.pow(3) * 19 + diff as usize;
                    if j >= 3 && seen[seq] != i as u16 {
                        seen[seq] = i as u16;
                        gain[seq] += next_num % 10;
                    }

                    num = next_num;
                }
                (part1 + u64::from(num), seen, gain)
            },
        )
        .map(|(part1, _, gain)| (part1, gain))
        .reduce(
            || (0, vec![0_u32; NUM_SEQ]),
            |(part1_1, mut gain1), (part1_2, gain2)| {
                for (a, b) in gain1.iter_mut().zip(gain2) {
                    *a += b;
                }
                (part1_1 + part1_2, gain1)
            },
        );
    let part2 = gain.into_iter().max().unwrap();

    (part1, part2)
}

fn next(mut x: u32) -> u32 {
    x = (x ^ (x << 6)) & 0xFF_FFFF;
    x ^= x >> 5;
    (x ^ (x << 11)) & 0xFF_FFFF
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn next() {
        let expected = [
            123, 15_887_950, 16_495_136, 527_345, 704_524, 1_553_684, 12_683_156, 11_100_544,
            12_249_484, 7_753_432, 5_908_254,
        ];
        for (from, to) in expected.into_iter().tuple_windows() {
            assert_eq!(super::next(from), to, "{from} -> {to}");
        }
    }

    #[test]
    fn part1() {
        assert_eq!(run("1\n10\n100\n2024").0, 37_327_623);
    }

    #[test]
    fn part2() {
        assert_eq!(run("1\n2\n3\n2024").1, 23);
    }
}
