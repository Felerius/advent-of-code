use anyhow::Result;

const NUM_SEQ: usize = 19_usize.pow(4);

pub fn run(input: &str) -> Result<(u64, u32)> {
    let mut seen = vec![u16::MAX; NUM_SEQ];
    let mut gain = vec![0_u32; NUM_SEQ];
    let part1 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
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
            u64::from(num)
        })
        .sum();
    let part2 = gain.into_iter().max().unwrap();

    Ok((part1, part2))
}

fn next(mut x: u32) -> u32 {
    x = (x ^ (x << 6)) & 0xFFFFFF;
    x ^= x >> 5;
    (x ^ (x << 11)) & 0xFFFFFF
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn next() {
        let expected = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];
        for (from, to) in expected.into_iter().tuple_windows() {
            assert_eq!(super::next(from), to, "{from} -> {to}");
        }
    }

    #[test]
    fn part1() {
        assert_eq!(run("1\n10\n100\n2024").unwrap().0, 37327623);
    }

    #[test]
    fn part2() {
        assert_eq!(run("1\n2\n3\n2024").unwrap().1, 23);
    }
}
