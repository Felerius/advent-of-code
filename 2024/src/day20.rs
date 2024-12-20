use anyhow::Result;

pub fn run(input: &str) -> Result<(usize, usize)> {
    run_testable(input, 100)
}

fn run_testable(input: &str, shortcut_threshold: usize) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let (sy, sx) = itertools::iproduct!(0..height, 0..width)
        .find(|&(y, x)| grid[y][x] == b'S')
        .unwrap();

    let mut dist = vec![vec![usize::MAX; width]; height];
    dist[sy][sx] = 0;
    let (mut y, mut x) = (sy, sx);
    while grid[y][x] != b'E' {
        let (y2, x2) = [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .into_iter()
            .find(|&(y2, x2)| grid[y2][x2] != b'#' && dist[y2][x2] == usize::MAX)
            .unwrap();
        dist[y2][x2] = dist[y][x] + 1;
        (y, x) = (y2, x2);
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for (y, x) in itertools::iproduct!(0..height, 0..width).filter(|&(y, x)| grid[y][x] != b'#') {
        let low_y = y.saturating_sub(20);
        let high_y = (y + 21).min(height);
        for y2 in low_y..high_y {
            let rem = 20 - y.abs_diff(y2);
            let low_x = x.saturating_sub(rem);
            let high_x = (x + rem + 1).min(width);
            for x2 in low_x..high_x {
                let shortcut_dist = y.abs_diff(y2) + x.abs_diff(x2);
                let saved = dist[y2][x2]
                    .saturating_sub(shortcut_dist)
                    .saturating_sub(dist[y][x]);
                if grid[y2][x2] != b'#' && saved >= shortcut_threshold {
                    part2 += 1;
                    part1 += usize::from(shortcut_dist == 2);
                }
            }
        }
    }

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1() {
        let (part1, _) = run_testable(INPUT, 12).unwrap();
        assert_eq!(part1, 8);
    }

    #[test]
    fn part2() {
        let (_, part2) = run_testable(INPUT, 68).unwrap();
        assert_eq!(part2, 14 + 12 + 22 + 4 + 3);
    }
}
