use anyhow::Result;
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(usize, u64)> {
    run_inner(input, 1000, sort_parallel)
}

#[register]
fn sequential(input: &str) -> Result<(usize, u64)> {
    run_inner(input, 1000, sort_sequential)
}

fn run_inner(
    input: &str,
    count: usize,
    sort: impl FnOnce(&[[u32; 3]]) -> Vec<(usize, usize)>,
) -> Result<(usize, u64)> {
    let points: Vec<[u32; 3]> = input
        .lines()
        .map(Input::unsigned_integers_n)
        .try_collect()?;
    let pairs = sort(&points);

    let mut dsu = DisjointSetUnion::new(points.len());
    let mut components = points.len();
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, (x, y)) in pairs.into_iter().enumerate() {
        components -= usize::from(dsu.join(x, y));
        if i + 1 == count {
            part1 = dsu.root_sizes().k_largest(3).product();
        }
        if components == 1 {
            debug_assert!(i + 1 >= count);
            part2 = u64::from(points[x][0]) * u64::from(points[y][0]);
            break;
        }
    }

    Ok((part1, part2))
}

fn sort_sequential(points: &[[u32; 3]]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<_> = (0..points.len()).tuple_combinations().collect();
    pairs.sort_unstable_by_key(|&(i, j)| sqr_dist(points[i], points[j]));
    pairs
}

fn sort_parallel(points: &[[u32; 3]]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<_> = (0..points.len()).tuple_combinations().collect();
    pairs.par_sort_unstable_by_key(|&(i, j)| sqr_dist(points[i], points[j]));
    pairs
}

fn sqr_dist([x1, y1, z1]: [u32; 3], [x2, y2, z2]: [u32; 3]) -> u64 {
    u64::from(x1.abs_diff(x2)).pow(2)
        + u64::from(y1.abs_diff(y2)).pow(2)
        + u64::from(z1.abs_diff(z2)).pow(2)
}

struct DisjointSetUnion(Vec<isize>);

impl DisjointSetUnion {
    fn new(size: usize) -> Self {
        DisjointSetUnion(vec![-1; size])
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

    fn root_sizes(&self) -> impl Iterator<Item = usize> {
        self.0.iter().filter(|&&x| x < 0).map(|&x| (-x) as usize)
    }

    fn join(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return false;
        }
        let (large, small) = if self.0[i] < self.0[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.0[large] += self.0[small];
        self.0[small] = large as isize;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test() {
        assert_eq!(run_inner(INPUT, 10, sort_parallel).unwrap(), (40, 25_272));
        assert_eq!(run_inner(INPUT, 10, sort_sequential).unwrap(), (40, 25_272));
    }
}
