use std::{array, collections::VecDeque};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::knot_hash;

pub(crate) fn run(input: &str) -> (usize, usize) {
    let input = input.trim();
    let mut grid = compute_grid_rayon(input);
    let part1 = grid.iter().flatten().filter(|&&b| b).count();

    let mut queue = VecDeque::with_capacity(128 * 128);
    let mut part2 = 0;
    for (x0, y0) in itertools::iproduct!(0..128, 0..128) {
        if !grid[x0][y0] {
            continue;
        }

        part2 += 1;
        grid[x0][y0] = false;
        queue.push_back((x0, y0));
        while let Some((x, y)) = queue.pop_front() {
            let neighbors = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            for (nx, ny) in neighbors {
                if nx < 128 && ny < 128 && grid[nx][ny] {
                    grid[nx][ny] = false;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    (part1, part2)
}

fn compute_grid_rayon(input: &str) -> [[bool; 128]; 128] {
    let mut grid = [[false; 128]; 128];
    grid.par_iter_mut().enumerate().for_each(|(i, row)| {
        let num = knot_hash::hash_str(&format!("{input}-{i}"));
        *row = array::from_fn(|j| ((num >> (127 - j)) & 1) != 0);
    });
    grid
}

#[allow(dead_code, reason = "alternative solution")]
fn compute_grid_single_threaded(input: &str) -> [[bool; 128]; 128] {
    array::from_fn(|i| {
        let num = knot_hash::hash_str(&format!("{input}-{i}"));
        array::from_fn(|j| ((num >> (127 - j)) & 1) != 0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("flqrgnkx"), (8108, 1242));
    }
}
