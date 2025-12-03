use std::iter;

use anyhow::Result;
use register::register;

const N1: usize = 2017;
const N2: usize = 50_000_000;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let jump = input.trim().parse::<usize>()? + 1;

    // Part 1: we can reverse engineer the process without ever materializing
    // the array. We imagine the final array rotated such that the 2017 is the
    // last element and then simulate undoing the insertions backwards, tracking
    // just their indices. The last insertions at index 0, which will be the
    // first one we find, will give us the answer (i.e., the value after 2017 in
    // the final array).
    let mut insertion_indices = iter::successors(Some((N1, N1)), |(prev_num, prev_idx)| {
        let size = prev_num;
        let idx = (prev_idx + size - (jump % size)) % size;
        Some((size - 1, idx))
    });
    let part1 = insertion_indices
        .find_map(|(size, idx)| (idx == 0).then_some(size))
        .unwrap();

    // Part 2 is similar, but actually far easier. We can simulate forwards, and
    // don't need to rotate the array. Any time we insert after index 0, this
    // value becomes the new successor of the value 0. Thus we need to find the
    // last such value. Since we only care about insertions at index 0, we can
    // skip any insertions that can't be at index 0.
    let insertion_indices = iter::successors(Some((0, 0)), |&(prev_num, prev_idx)| {
        // At small sizes just simulate normally to avoid edge cases
        let size: usize = prev_num + 1;
        if size <= jump + 5 {
            let idx = (prev_idx + jump) % size;
            Some((size, idx))
        } else {
            let skip = (size - prev_idx).div_ceil(jump);
            let num = size + skip - 1;
            let idx = prev_idx + skip * jump;
            let idx = idx - usize::from(idx >= num) * num;
            Some((num, idx))
        }
    });
    let (part2, _) = insertion_indices
        .take_while(|&(num, _)| num <= N2)
        .filter(|&(_, idx)| idx == 0)
        .last()
        .unwrap();

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("3").unwrap().0, 638);
    }
}
