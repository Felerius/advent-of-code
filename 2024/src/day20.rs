use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub(crate) fn run(input: &str) -> (usize, usize) {
    run_testable(input, 100)
}

fn run_testable(input: &str, shortcut_threshold: usize) -> (usize, usize) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let height = grid.len();
    let width = grid[0].len();
    let (sy, sx) = itertools::iproduct!(0..height, 0..width)
        .find(|&(y, x)| grid[y][x] == b'S')
        .unwrap();

    let mut dist = vec![vec![usize::MAX; width]; height];
    let mut path = vec![(sy, sx)];
    dist[sy][sx] = 0;
    let (mut y, mut x) = (sy, sx);
    while grid[y][x] != b'E' {
        let (y2, x2) = [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .into_iter()
            .find(|&(y2, x2)| grid[y2][x2] != b'#' && dist[y2][x2] == usize::MAX)
            .unwrap();
        dist[y2][x2] = dist[y][x] + 1;
        path.push((y2, x2));
        (y, x) = (y2, x2);
    }

    path.into_par_iter()
        .map(|(y, x)| {
            let low_y = y.saturating_sub(20);
            let high_y = (y + 21).min(height);
            (low_y..high_y)
                .flat_map(move |y2| {
                    let rem = 20 - y.abs_diff(y2);
                    let low_x = x.saturating_sub(rem);
                    let high_x = (x + rem + 1).min(width);
                    (low_x..high_x).map(move |x2| (y2, x2))
                })
                .filter(|&(y2, x2)| grid[y2][x2] != b'#')
                .filter_map(|(y2, x2)| {
                    let shortcut_dist = y.abs_diff(y2) + x.abs_diff(x2);
                    let saved = dist[y2][x2]
                        .saturating_sub(shortcut_dist)
                        .saturating_sub(dist[y][x]);
                    (saved >= shortcut_threshold).then_some(shortcut_dist)
                })
                .fold((0, 0), |(part1, part2), dist| {
                    (part1 + usize::from(dist == 2), part2 + 1)
                })
        })
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
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
        let (part1, _) = run_testable(INPUT, 12);
        assert_eq!(part1, 8);
    }

    #[test]
    fn part2() {
        let (_, part2) = run_testable(INPUT, 68);
        assert_eq!(part2, 14 + 12 + 22 + 4 + 3);
    }
}
