use anyhow::Result;
use array_const_fn_init::array_const_fn_init;
use utils::hash::{FastHashCollectionExt, FastHashMap};

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut cache = FastHashMap::new();
    let (part1, part2) = input
        .split_ascii_whitespace()
        .map(|s| {
            let num: u64 = s.parse().unwrap();
            let part1 = simulate(num, 25, &mut cache);
            let part2 = simulate(num, 75, &mut cache);
            (part1, part2)
        })
        .fold((0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2));

    Ok((part1, part2))
}

fn simulate(num: u64, steps: u8, cache: &mut FastHashMap<(u64, u8), usize>) -> usize {
    if steps == 0 {
        return 1;
    }
    if let Some(&cached) = cache.get(&(num, steps)) {
        return cached;
    }

    let result = if num == 0 {
        simulate(1, steps - 1, cache)
    } else if num.ilog10() % 2 == 1 {
        let len = num.ilog10() as usize + 1;
        simulate(num / TEN_POW[len / 2], steps - 1, cache)
            + simulate(num % TEN_POW[len / 2], steps - 1, cache)
    } else {
        simulate(2024 * num, steps - 1, cache)
    };
    cache.insert((num, steps), result);
    result
}

const TEN_POW: [u64; 20] = array_const_fn_init![ten_pow; 20];

const fn ten_pow(i: usize) -> u64 {
    10_u64.pow(i as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("125 17").unwrap().0, 55312);
    }
}
