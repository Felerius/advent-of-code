use std::collections::VecDeque;

use anyhow::Result;
use register::register;

#[register]
fn run(input: &str) -> Result<(u8, usize)> {
    let num: usize = input.parse()?;
    let mut grid = [[false; 51]; 51];
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let n = x * x + 3 * x + 2 * x * y + y + y * y + num;
            *cell = n.count_ones() % 2 == 1;
        }
    }

    let mut queue = VecDeque::from_iter([(1, 1)]);
    let mut dist = [[u8::MAX; 51]; 51];
    let mut seen = 1;
    dist[1][1] = 0;

    while let Some((y, x)) = queue.pop_front() {
        if dist[39][31] != u8::MAX && dist[y][x] >= 50 {
            break;
        }

        let neighbors = [
            y.checked_sub(1).map(|y2| (y2, x)),
            x.checked_sub(1).map(|x2| (y, x2)),
            Some((y, x + 1)),
            Some((y + 1, x)),
        ];
        for (y2, x2) in neighbors.into_iter().flatten() {
            if !grid[y2][x2] && dist[y2][x2] == u8::MAX {
                dist[y2][x2] = dist[y][x] + 1;
                if dist[y2][x2] <= 50 {
                    seen += 1;
                }
                queue.push_back((y2, x2));
            }
        }
    }

    Ok((dist[39][31], seen))
}
