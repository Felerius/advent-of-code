use std::{cmp::Reverse, collections::BinaryHeap, mem};

use anyhow::Result;
use itertools::Itertools;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    run_inner(input, part2_remainders_list)
}

#[register]
fn dijkstra(input: &str) -> Result<(usize, usize)> {
    run_inner(input, part2_dijkstra)
}

#[register]
fn brute_force(input: &str) -> Result<(usize, usize)> {
    run_inner(input, part2_brute_force)
}

fn run_inner(input: &str, part2: fn(&[[usize; 2]]) -> usize) -> Result<(usize, usize)> {
    let mut layers: Vec<_> = input
        .lines()
        .map(Input::unsigned_integers_n::<usize, 2>)
        .try_collect()?;
    layers.sort_unstable_by_key(|&[_, range]| range);

    let part1 = layers
        .iter()
        .filter(|[depth, range]| depth % (2 * range - 2) == 0)
        .map(|[depth, range]| depth * range)
        .sum();
    let part2 = part2(&layers);

    Ok((part1, part2))
}

fn part2_remainders_list(layers: &[[usize; 2]]) -> usize {
    let mut current = vec![1];
    let mut next = vec![];
    let mut lcm = 1;

    for &[depth, range] in layers {
        let modulus = 2 * range - 2;
        let next_lcm = num::integer::lcm(lcm, modulus);
        let blocked = (modulus - (depth % modulus)) % modulus;
        for i in 0..next_lcm / lcm {
            for &delay in &current {
                let next_delay = delay + i * lcm;
                if next_delay % modulus != blocked {
                    next.push(next_delay);
                }
            }
        }

        lcm = next_lcm;
        current.clear();
        mem::swap(&mut current, &mut next);
    }

    current[0]
}

fn part2_dijkstra(layers: &[[usize; 2]]) -> usize {
    let mut lcm = 1;
    let precalc: Vec<_> = layers
        .iter()
        .map(|&[depth, range]| {
            let modulus = 2 * range - 2;
            let blocked = (modulus - (depth % modulus)) % modulus;
            let prev_lcm = lcm;
            lcm = num::integer::lcm(lcm, modulus);
            (modulus, blocked, prev_lcm, lcm / prev_lcm)
        })
        .collect();

    let mut heap = BinaryHeap::from_iter([(Reverse(1), 0)]);
    while let Some((Reverse(delay), index)) = heap.pop() {
        if index == layers.len() {
            return delay;
        }

        let (modulus, blocked, width, degree) = precalc[index];
        for i in 0..degree {
            let next_delay = delay + i * width;
            if next_delay % modulus != blocked {
                heap.push((Reverse(next_delay), index + 1));
            }
        }
    }

    unreachable!("no solution found")
}

fn part2_brute_force(layers: &[[usize; 2]]) -> usize {
    #[expect(clippy::maybe_infinite_iter)]
    (0..)
        .find(|delay| {
            layers
                .iter()
                .all(|[depth, range]| (depth + delay) % (2 * range - 2) != 0)
        })
        .expect("infinite iterator should never end")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test_name() {
        assert_eq!(run(INPUT).unwrap(), (24, 10));
        assert_eq!(dijkstra(INPUT).unwrap(), (24, 10));
        assert_eq!(brute_force(INPUT).unwrap(), (24, 10));
    }
}
