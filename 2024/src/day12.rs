use std::mem;

use anyhow::Result;
use utils::hash::{FastHashCollectionExt, FastHashSet};

pub fn run(input: &str) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut seen = vec![vec![false; width]; height];
    let mut queue = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for (y0, x0) in itertools::iproduct!(0..height, 0..width) {
        if seen[y0][x0] {
            continue;
        }

        seen[y0][x0] = true;
        queue.push((y0, x0));
        let mut queue_idx = 0;
        while let Some(&(y, x)) = queue.get(queue_idx) {
            queue_idx += 1;
            for (y2, x2) in neighbors(y, x, &grid) {
                if !seen[y2][x2] && grid[y2][x2] == grid[y][x] {
                    seen[y2][x2] = true;
                    queue.push((y2, x2));
                }
            }
        }

        let (perimeter1, sides1) = calc_sides(&mut queue);
        for (y, x) in &mut queue {
            mem::swap(y, x);
        }
        let (perimeter2, sides2) = calc_sides(&mut queue);
        let area = queue.len();
        queue.clear();

        part1 += area * (perimeter1 + perimeter2);
        part2 += area * (sides1 + sides2);
    }

    Ok((part1, part2))
}

fn calc_sides(pts: &mut [(usize, usize)]) -> (usize, usize) {
    pts.sort_unstable();
    let mut cur = FastHashSet::new();
    let mut prev = FastHashSet::new();
    let mut prev_y = usize::MAX - 1;
    let mut perimeter = 0;
    let mut sides = 0;
    for row in pts.chunk_by(|&(y1, _), &(y2, _)| y1 == y2) {
        let cur_y = row[0].0;
        if prev_y + 1 != cur_y {
            prev.clear();
        }
        prev_y = cur_y;

        cur.clear();
        let mut prev_x = None;
        let mut parity = true;
        for &(_, x) in row {
            if prev_x.is_none_or(|prev_x| prev_x + 1 != x) {
                if let Some(prev_x) = prev_x {
                    cur.insert((prev_x + 1, parity));
                    parity = !parity;
                }

                cur.insert((x, parity));
                parity = !parity;
            }

            prev_x = Some(x);
        }

        if let Some(prev_x) = prev_x {
            cur.insert((prev_x + 1, parity));
        }

        sides += cur.difference(&prev).count();
        perimeter += cur.len();
        mem::swap(&mut cur, &mut prev);
    }

    (perimeter, sides)
}

fn neighbors(y: usize, x: usize, grid: &[&[u8]]) -> impl Iterator<Item = (usize, usize)> {
    [
        y.checked_sub(1).map(|yi| (yi, x)),
        x.checked_sub(1).map(|xi| (y, xi)),
        (y + 1 < grid.len()).then_some((y + 1, x)),
        (x + 1 < grid[0].len()).then_some((y, x + 1)),
    ]
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const INPUT2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const INPUT3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const INPUT4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const INPUT5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT1).unwrap().0, 140);
        assert_eq!(run(INPUT2).unwrap().0, 772);
        assert_eq!(run(INPUT3).unwrap().0, 1930);
    }

    #[test]
    fn part2() {
        assert_eq!(run(INPUT1).unwrap().1, 80);
        assert_eq!(run(INPUT2).unwrap().1, 436);
        assert_eq!(run(INPUT4).unwrap().1, 236);
        assert_eq!(run(INPUT5).unwrap().1, 368);
        assert_eq!(run(INPUT3).unwrap().1, 1206);
    }
}
