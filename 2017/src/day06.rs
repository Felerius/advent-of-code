use std::{cmp::Reverse, iter};

use anyhow::Result;
use arrayvec::ArrayVec;
use itertools::Itertools;
use register::register;
use utils::hash::{FastHashCollectionExt, FastHashMap};

type State = ArrayVec<u8, 16>;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let initial_state: State = input
        .split_ascii_whitespace()
        .map(str::parse)
        .try_collect()?;
    let mut seen = FastHashMap::new();
    let (part1, part2) = generate_states(initial_state)
        .enumerate()
        .find_map(|(i, state)| seen.insert(state, i).map(|j| (i, i - j)))
        .unwrap();
    Ok((part1, part2))
}

fn generate_states(initial_state: State) -> impl Iterator<Item = State> {
    iter::successors(Some(initial_state), |state| {
        let mut state = state.clone();
        let (i, _) = state
            .iter()
            .enumerate()
            .max_by_key(|&(i, &x)| (x, Reverse(i)))
            .unwrap();
        let len = state.len();
        let amount = state[i];
        state[i] = 0;
        for j in 0..len {
            let more = j < usize::from(amount) % len;
            state[(i + 1 + j) % len] += amount / len as u8 + u8::from(more);
        }
        Some(state)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (part1, part2) = run("0 2 7 0").unwrap();
        assert_eq!(part1, 5);
        assert_eq!(part2, 4);
    }
}
