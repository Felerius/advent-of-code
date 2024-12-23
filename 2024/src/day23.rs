use std::{
    array,
    fmt::{self, Display, Formatter},
};

use anyhow::Result;
use itertools::Itertools;
use tinybitset::TinyBitSet;

const N: usize = 26 * 26;

type AdjBitset = TinyBitSet<u64, 11>;

pub fn run(input: &str) -> Result<(usize, String)> {
    let mut adj: [_; N] = array::from_fn(|i| AdjBitset::singleton(i));
    for line in input.lines() {
        let line = line.as_bytes();
        let v1 = parse_node(line[0], line[1]);
        let v2 = parse_node(line[3], line[4]);
        adj[v1].insert(v2);
        adj[v2].insert(v1);
    }

    let v_ta = parse_node(b't', b'a');
    let v_ua = parse_node(b'u', b'a');
    let part1 = (v_ta..v_ua)
        .map(|v1| {
            adj[v1]
                .iter()
                .filter(|v2| !(v1..v_ua).contains(v2))
                .tuple_combinations()
                .filter(|&(v2, v3)| adj[v2][v3])
                .count()
        })
        .sum();

    let mut state = State {
        adj: &adj,
        clique: Vec::new(),
        largest: Vec::new(),
    };
    enumerate_cliques(&mut state, (0..N).collect());
    let part2 = state.largest.into_iter().map(PrintNode).join(",");

    Ok((part1, part2))
}

struct State<'a> {
    adj: &'a [AdjBitset; N],
    clique: Vec<usize>,
    largest: Vec<usize>,
}

fn enumerate_cliques(state: &mut State<'_>, mut ready: AdjBitset) {
    let last = state.clique.last().copied();
    if state.clique.len() > state.largest.len() {
        state.largest.clear();
        state.largest.extend(state.clique.iter().copied());
    }
    if state.clique.len() + ready.len() <= state.largest.len() {
        return;
    }

    for next in ready {
        debug_assert!(Some(next) > last);
        ready.remove(next);

        let next_ready = ready & state.adj[next];
        state.clique.push(next);
        enumerate_cliques(state, next_ready);
        state.clique.pop();
    }
}

fn parse_node(c1: u8, c2: u8) -> usize {
    usize::from(c1 - b'a') * 26 + usize::from(c2 - b'a')
}

struct PrintNode(usize);

impl Display for PrintNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c1 = char::from((self.0 / 26) as u8 + b'a');
        let c2 = char::from((self.0 % 26) as u8 + b'a');
        write!(f, "{c1}{c2}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).unwrap().0, 7);
    }

    #[test]
    fn part2() {
        assert_eq!(run(INPUT).unwrap().1, "co,de,ka,ta");
    }
}
