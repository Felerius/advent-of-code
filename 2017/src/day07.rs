use std::ops::ControlFlow;

use anyhow::{bail, Context, Result};
use utils::hash::Indexer;

pub(crate) fn run(input: &str) -> Result<(String, u32)> {
    let mut indices = Indexer::new();
    let mut adj = vec![];
    let mut weights = vec![];
    let mut is_child = vec![];
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        let name = parts.next().context("invalid line: missing name")?;
        let v = indices.index_of(name);
        if adj.len() < indices.len() {
            adj.resize_with(indices.len(), Vec::new);
            weights.resize(indices.len(), 0);
        }
        if v == adj.len() {
            adj.push(vec![]);
            weights.push(0);
        }

        let weight_str = parts.next().context("invalid line: missing weight")?;
        let weight: u32 = weight_str[1..weight_str.len() - 1]
            .parse()
            .context("invalid line: invalid weight")?;
        weights[v] = weight;

        let child_indices = parts
            .skip(1)
            .map(|child| indices.index_of(child.trim_end_matches(',')))
            .inspect(|&v2| {
                if is_child.len() <= v2 {
                    is_child.resize(v2 + 1, false);
                }
                is_child[v2] = true;
            });
        adj[v].extend(child_indices);
    }

    let n = indices.len();
    let root = (0..n)
        .find(|&v| !is_child[v])
        .context("graph is not a tree")?;
    let part1 = *indices
        .as_map()
        .iter()
        .find(|&(_, &v)| v == root)
        .unwrap()
        .0;
    let ControlFlow::Break(part2) = dfs(root, &adj, &weights) else {
        bail!("tree is already balanced");
    };

    Ok((part1.to_string(), part2))
}

fn dfs(v: usize, adj: &[Vec<usize>], weights: &[u32]) -> ControlFlow<u32, (usize, u32)> {
    let d = adj[v].len();
    let mut sum = weights[v];
    let mut children = Vec::with_capacity(d);
    for &v2 in &adj[v] {
        let (cand, w) = dfs(v2, adj, weights)?;
        sum += w;
        children.push((cand, w));
    }

    children.sort_unstable_by_key(|&(_, w)| w);
    if d == 0 || children[0].1 == children[d - 1].1 {
        return ControlFlow::Continue((v, sum));
    }

    if children[0].1 == children[1].1 {
        children.reverse();
    }
    let (cand, wrong_weight) = children[0];
    let expected_weight = weights[cand] + children[1].1 - wrong_weight;
    ControlFlow::Break(expected_weight)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test() {
        let (part1, part2) = run(INPUT).unwrap();
        assert_eq!(part1, "tknk");
        assert_eq!(part2, 60);
    }
}
