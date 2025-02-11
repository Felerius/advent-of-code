use anyhow::{Context, Result};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let n = input.lines().count();
    let mut dsu = DisjointSetUnion::new(n);
    for (i, line) in input.lines().enumerate() {
        debug_assert!(line.starts_with(&format!("{i} <-> ")));
        let (_, to_str) = line.split_once(" <-> ").context("invalid input line")?;
        for to in to_str.split(", ") {
            let to = to.parse().context("invalid number")?;
            dsu.union(i, to);
        }
    }

    let part1 = dsu.group_size(0);
    let part2 = (0..n).filter(|&i| dsu.find(i) == i).count();
    Ok((part1, part2))
}

struct DisjointSetUnion(Vec<isize>);

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self(vec![-1; n])
    }

    fn find(&mut self, i: usize) -> usize {
        if let Ok(parent) = usize::try_from(self.0[i]) {
            let root = self.find(parent);
            self.0[i] = root as isize;
            root
        } else {
            i
        }
    }

    fn group_size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        -self.0[root] as usize
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let mut i = self.find(i);
        let mut j = self.find(j);
        if i == j {
            return false;
        }

        if self.0[i] > self.0[j] {
            (i, j) = (j, i);
        }
        self.0[i] += self.0[j];
        self.0[j] = i as isize;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (6, 2));
    }
}
