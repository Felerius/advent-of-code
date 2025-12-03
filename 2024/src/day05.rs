use anyhow::{Result, ensure};
use register::register;
use utils::input::Input;

// Assume that node indices in the input are in the range [0, N).
const N: usize = 100;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let mut lines = input.lines();
    let mut adj_rev = [0_u128; N];
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let [from, to] = line.unsigned_integers_n::<usize, 2>()?;
        ensure!(from < N && to < N, "node index out of bounds");
        adj_rev[to] |= 1 << from;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let nodes: Vec<_> = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        // The inputs seem to be designed s.t. the subgraph induced `nodes` is
        // a tournament graph, i.e., there is exactly one directed edge between
        // every pair of nodes. Combined with the fact that the subgraph must be
        // acyclic for the solution to exist, this means that the index of a node
        // in the unique topological ordering of the subgraph is given by its
        // in-degree.
        let mask = nodes.iter().map(|&v| 1 << v).sum::<u128>();
        let in_deg: Vec<_> = nodes
            .iter()
            .map(|&v| (adj_rev[v] & mask).count_ones() as usize)
            .collect();

        #[cfg(debug_assertions)]
        {
            let mut in_deg_sorted = in_deg.clone();
            in_deg_sorted.sort_unstable();
            for (i, d) in in_deg_sorted.into_iter().enumerate() {
                assert_eq!(i, d);
            }
        }

        let correct = in_deg.iter().enumerate().all(|(i, &d)| d == i);
        let median_index = nodes.len() / 2;
        if correct {
            part1 += nodes[median_index];
        } else {
            part2 += nodes
                .into_iter()
                .zip(in_deg)
                .find_map(|(v, d)| (d == median_index).then_some(v))
                .unwrap();
        }
    }

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (143, 123));
    }
}
