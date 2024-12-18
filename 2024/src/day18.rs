use std::collections::VecDeque;

use anyhow::Result;
use utils::input;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

pub fn run(input: &str) -> Result<(usize, String)> {
    let bytes: Vec<_> = input
        .lines()
        .map(|line| input::integers::<usize, 2>(line))
        .collect();
    let part1 = find_shortest_path(&bytes[..1024]).unwrap();

    let mut low = 1024;
    let mut high = bytes.len();
    while high - low > 1 {
        let mid = (low + high) / 2;
        if find_shortest_path(&bytes[..mid]).is_some() {
            low = mid;
        } else {
            high = mid;
        }
    }
    let [x, y] = bytes[low];
    let part2 = format!("{x},{y}");

    Ok((part1, part2))
}

fn find_shortest_path(bytes: &[[usize; 2]]) -> Option<usize> {
    let mut grid = [[false; WIDTH]; HEIGHT];
    for &[x, y] in bytes {
        grid[y][x] = true;
    }

    let mut dist = [[usize::MAX; WIDTH]; HEIGHT];
    let mut queue = VecDeque::from_iter([(0, 0)]);
    dist[0][0] = 0;
    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == (WIDTH - 1, HEIGHT - 1) {
            break;
        }

        let neighbors = [
            x.checked_sub(1).map(|xi| (xi, y)),
            y.checked_sub(1).map(|yi| (x, yi)),
            (x + 1 < WIDTH).then(|| (x + 1, y)),
            (y + 1 < HEIGHT).then(|| (x, y + 1)),
        ];
        for (x2, y2) in neighbors.into_iter().flatten() {
            if dist[y2][x2] == usize::MAX && !grid[y2][x2] {
                dist[y2][x2] = dist[y][x] + 1;
                queue.push_back((x2, y2));
            }
        }
    }

    let d = dist[HEIGHT - 1][WIDTH - 1];
    (d != usize::MAX).then_some(d)
}
