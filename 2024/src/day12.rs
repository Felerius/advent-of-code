use std::collections::VecDeque;

use anyhow::Result;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut seen = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for (y0, x0) in itertools::iproduct!(0..height, 0..width) {
        if seen[y0][x0] {
            continue;
        }

        seen[y0][x0] = true;
        queue.push_back((y0, x0));
        let mut area = 0;
        let mut corners = 0;
        let mut perimeter = 0;
        while let Some((y, x)) = queue.pop_front() {
            let neighbors = neighbors(y, x, &grid);
            let non_diagonal = [0, 2, 4, 6].map(|i| neighbors[i]);

            area += 1;
            perimeter += non_diagonal.iter().filter(|&&c| c.is_none()).count();

            let outside_corners = (0..4)
                .filter(|&i| non_diagonal[i].is_none() && non_diagonal[(i + 1) % 4].is_none())
                .count();
            let inside_corners = [1, 3, 5, 7]
                .into_iter()
                .filter(|&i| {
                    neighbors[i].is_none()
                        && neighbors[i - 1].is_some()
                        && neighbors[(i + 1) % 8].is_some()
                })
                .count();
            corners += outside_corners + inside_corners;

            for (y2, x2) in non_diagonal.into_iter().flatten() {
                if !seen[y2][x2] && grid[y2][x2] == grid[y][x] {
                    seen[y2][x2] = true;
                    queue.push_back((y2, x2));
                }
            }
        }

        part1 += area * perimeter;
        part2 += area * corners;
    }

    Ok((part1, part2))
}

fn neighbors(y: usize, x: usize, grid: &[&[u8]]) -> [Option<(usize, usize)>; 8] {
    let up = y > 0;
    let right = x + 1 < grid[0].len();
    let down = y + 1 < grid.len();
    let left = x > 0;
    [
        up.then(|| (y - 1, x)),
        (up && right).then(|| (y - 1, x + 1)),
        right.then(|| (y, x + 1)),
        (right && down).then(|| (y + 1, x + 1)),
        down.then(|| (y + 1, x)),
        (down && left).then(|| (y + 1, x - 1)),
        left.then(|| (y, x - 1)),
        (left && up).then(|| (y - 1, x - 1)),
    ]
    .map(|neigh| neigh.filter(|&(y2, x2)| grid[y2][x2] == grid[y][x]))
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
