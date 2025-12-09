use std::thread;

use anyhow::Result;
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(usize, u64)> {
    jarnik_prim_multi_threaded(input)
}

#[register]
fn kruskal_multi_threaded(input: &str) -> Result<(usize, u64)> {
    kruskal_impl(input, sort_indices_parallel)
}

#[register]
fn kruskal_single_threaded(input: &str) -> Result<(usize, u64)> {
    kruskal_impl(input, sort_indices_single_threaded)
}

#[register]
fn jarnik_prim_single_threaded(input: &str) -> Result<(usize, u64)> {
    let (points, part1_edge_count) = parse(input)?;

    let lightest_edges = (0..points.len())
        .tuple_combinations()
        .k_smallest_relaxed_by_key(part1_edge_count, |&(i, j)| sqr_dist(points[i], points[j]));
    let mut dsu = DisjointSetUnion::new(points.len());
    for (i, j) in lightest_edges {
        dsu.join(i, j);
    }
    let part1 = dsu.root_sizes().k_largest(3).product();

    let part2 = jarnik_prim_impl(&points);
    Ok((part1, part2))
}

#[register]
fn jarnik_prim_multi_threaded(input: &str) -> Result<(usize, u64)> {
    let (points, part1_edge_count) = parse(input)?;
    let n = points.len();

    let num_cores = num_cpus::get();
    assert!(num_cores >= 2);

    thread::scope(|s| {
        let part2_handle = s.spawn(|| jarnik_prim_impl(&points));

        let indices_per_core = n.div_ceil(num_cores - 1);
        let key = |(j, k)| sqr_dist(points[j], points[k]);
        let part1_handles: Vec<_> = (0..num_cores - 1)
            .map(|i| {
                s.spawn(move || {
                    let start = i * indices_per_core;
                    let end = ((i + 1) * indices_per_core).min(n);
                    (start..end)
                        .flat_map(|j| (j + 1..n).map(move |k| (j, k)))
                        .k_smallest_relaxed_by_key(part1_edge_count, |&p| key(p))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        let lightest_edges = part1_handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .k_smallest_relaxed_by_key(part1_edge_count, |&p| key(p));

        let mut dsu = DisjointSetUnion::new(n);
        for (i, j) in lightest_edges {
            dsu.join(i, j);
        }
        let part1 = dsu.root_sizes().k_largest(3).product();

        let part2 = part2_handle.join().unwrap();
        Ok((part1, part2))
    })
}

fn kruskal_impl(
    input: &str,
    sort_indices: impl FnOnce(&[[u32; 3]]) -> Vec<(usize, usize)>,
) -> Result<(usize, u64)> {
    let (points, part1_edge_count) = parse(input)?;
    let pairs = sort_indices(&points);

    let mut dsu = DisjointSetUnion::new(points.len());
    let mut components = points.len();
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, (x, y)) in pairs.into_iter().enumerate() {
        components -= usize::from(dsu.join(x, y));
        if i + 1 == part1_edge_count {
            part1 = dsu.root_sizes().k_largest(3).product();
        }
        if components == 1 {
            debug_assert!(i + 1 >= part1_edge_count);
            part2 = u64::from(points[x][0]) * u64::from(points[y][0]);
            break;
        }
    }

    Ok((part1, part2))
}

fn jarnik_prim_impl(points: &[[u32; 3]]) -> u64 {
    let mut heaviest_mst_edge = (0, 0);
    let mut candidates: Vec<_> = (1..points.len())
        .map(|i| (sqr_dist(points[0], points[i]), 0, i))
        .collect();
    for _ in 0..points.len() - 1 {
        let x = candidates.iter().position_min().unwrap();
        let (d, from, to) = candidates.swap_remove(x);
        let x_product = u64::from(points[from][0]) * u64::from(points[to][0]);
        heaviest_mst_edge = heaviest_mst_edge.max((d, x_product));
        for (d2, from2, to2) in &mut candidates {
            let new_d = sqr_dist(points[to], points[*to2]);
            if new_d < *d2 {
                *d2 = new_d;
                *from2 = to;
            }
        }
    }

    heaviest_mst_edge.1
}

fn sort_indices_single_threaded(points: &[[u32; 3]]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<_> = (0..points.len()).tuple_combinations().collect();
    pairs.sort_unstable_by_key(|&(i, j)| sqr_dist(points[i], points[j]));
    pairs
}

fn sort_indices_parallel(points: &[[u32; 3]]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<_> = (0..points.len()).tuple_combinations().collect();
    pairs.par_sort_unstable_by_key(|&(i, j)| sqr_dist(points[i], points[j]));
    pairs
}

fn sqr_dist([x1, y1, z1]: [u32; 3], [x2, y2, z2]: [u32; 3]) -> u64 {
    u64::from(x1.abs_diff(x2)).pow(2)
        + u64::from(y1.abs_diff(y2)).pow(2)
        + u64::from(z1.abs_diff(z2)).pow(2)
}

fn parse(input: &str) -> Result<(Vec<[u32; 3]>, usize)> {
    let points: Vec<[u32; 3]> = input
        .lines()
        .map(Input::unsigned_integers_n)
        .try_collect()?;
    // Hack to avoid having to split functions for "main" vs. test path
    let part1_edge_count = if points.len() < 100 { 10 } else { 1000 };
    Ok((points, part1_edge_count))
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
        let impls: &[(_, fn(_) -> _)] = &[
            ("kruskal parallel", |i| {
                kruskal_impl(i, sort_indices_parallel)
            }),
            ("kruskal single-threaded", |i| {
                kruskal_impl(i, sort_indices_single_threaded)
            }),
            ("jarnik prim single-threaded", jarnik_prim_single_threaded),
            ("jarnik prim multi-threaded", jarnik_prim_multi_threaded),
        ];

        for (name, solve) in impls {
            let result = solve(INPUT).unwrap();
            assert_eq!(result, (40, 25_272), "Failed for {name}",);
        }
    }
}
