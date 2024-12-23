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
    let mut adj_matrix = [AdjBitset::new(); N];
    let mut adj: [_; N] = array::from_fn(|_| Vec::new());
    for i in 0..N {
        adj_matrix[i].insert(i);
    }
    for line in input.lines() {
        let line = line.as_bytes();
        let v1 = parse_node(line[0], line[1]);
        let v2 = parse_node(line[3], line[4]);
        adj_matrix[v1].insert(v2);
        adj_matrix[v2].insert(v1);
        adj[v1].push(v2);
        adj[v2].push(v1);
    }
    for row in &mut adj {
        row.sort_unstable();
    }

    let v_ta = parse_node(b't', b'a');
    let v_ua = parse_node(b'u', b'a');
    let part1 = (v_ta..v_ua)
        .map(|v1| {
            adj[v1]
                .iter()
                .copied()
                .filter(|v2| !(v1..v_ua).contains(v2))
                .tuple_combinations()
                .filter(|&(v2, v3)| adj_matrix[v2][v3])
                .count()
        })
        .sum();

    let part2_bs = (0..N)
        .flat_map(|v1| {
            let candidates: AdjBitset = adj_matrix[v1].iter().filter(|&v2| v2 > v1).collect();
            (0..1 << candidates.len()).filter_map(move |mask| {
                let sub_candidates: AdjBitset = candidates
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| mask & 1 << i != 0)
                    .map(|(_, v)| v)
                    .collect();
                let expected = sub_candidates.inserted(v1);
                let adj_union = sub_candidates
                    .iter()
                    .fold(expected, |acc, v2| acc & adj_matrix[v2]);

                (adj_union == expected).then_some(adj_union)
            })
        })
        .max_by_key(|bs| bs.len())
        .unwrap();
    let part2 = part2_bs.iter().map(PrintNode).join(",");

    Ok((part1, part2))
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
