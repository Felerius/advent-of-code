use std::ops::Range;

use anyhow::{Context, Result};
use itertools::Itertools;
use ndarray::{Array2, s};
use register::register;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').context("invalid input")?;
            anyhow::Ok((x.parse()?, y.parse()?))
        })
        .try_collect()?;
    assert!(points.len() > 1);
    assert!(points.len().is_multiple_of(2));
    if points[0].1 != points[1].1 {
        points.rotate_left(1);
    }

    let xses = compressed(points.iter().map(|&(x, _)| x));
    let yses = compressed(points.iter().map(|&(_, y)| y));
    for (x, y) in &mut points {
        *x = compress(*x, &xses);
        *y = compress(*y, &yses);
    }

    let mut cells = Array2::zeros((yses.len(), xses.len()));
    for (&(x1, y), &(x2, _)) in points.iter().tuple_windows() {
        cells.slice_mut(s![y, x1.min(x2)..x1.max(x2)]).fill(1);
    }
    for y in 1..yses.len() {
        for x in 0..xses.len() {
            cells[(y, x)] ^= cells[(y - 1, x)];
        }
    }
    for mut row in cells.rows_mut() {
        for x in (1..xses.len()).rev() {
            row[x - 1] += row[x];
        }
    }
    for y in (1..yses.len()).rev() {
        for x in 0..xses.len() {
            cells[(y - 1, x)] += cells[(y, x)];
        }
    }

    let (part1, part2) = points
        .iter()
        .copied()
        .tuple_combinations()
        .filter(|&((x1, y1), (x2, y2))| x1 != x2 && y1 != y2)
        .fold((0, 0), |(mut part1, mut part2), ((x1, y1), (x2, y2))| {
            let xl = x1.min(x2);
            let xh = x1.max(x2);
            let yl = y1.min(y2);
            let yh = y1.max(y2);
            let area = (xses[xh] - xses[xl] + 1) * (yses[yh] - yses[yl] + 1);

            part1 = part1.max(area);
            if rect_sum(&cells, xl..xh, yl..yh) == (xh - xl) * (yh - yl) {
                part2 = part2.max(area);
            }

            (part1, part2)
        });

    Ok((part1, part2))
}

fn compressed(coords: impl IntoIterator<Item = usize>) -> Vec<usize> {
    coords.into_iter().sorted_unstable().dedup().collect()
}

fn compress(coord: usize, compressed: &[usize]) -> usize {
    compressed.binary_search(&coord).unwrap()
}

fn rect_sum(cells: &Array2<usize>, x: Range<usize>, y: Range<usize>) -> usize {
    cells[(y.start, x.start)] + cells[(y.end, x.end)]
        - cells[(y.start, x.end)]
        - cells[(y.end, x.start)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn test() {
        assert_eq!(run(INPUT).unwrap(), (50, 24));
    }
}
