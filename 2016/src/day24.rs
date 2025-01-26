use std::collections::VecDeque;

use utils::num::bits;

pub(crate) fn run(input: &str) -> (usize, usize) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut pos = [(0, 0); 10];
    let mut num_pos = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                let num = cell - b'0';
                pos[usize::from(num)] = (y, x);
                num_pos = num_pos.max(num + 1);
            }
        }
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut which_pos = vec![u8::MAX; height * width];
    for (i, &(y, x)) in pos.iter().enumerate() {
        which_pos[y * width + x] = i as u8;
    }

    let mut queue = VecDeque::with_capacity(height * width);
    let mut seen = vec![u8::MAX; height * width];
    let mut dist = [[0; 10]; 10];
    for v0 in 0..num_pos {
        let (y0, x0) = pos[usize::from(v0)];
        queue.push_back((y0, x0, 0));
        seen[y0 * width + x0] = v0;
        while let Some((y, x, d)) = queue.pop_front() {
            if which_pos[y * width + x] != u8::MAX {
                dist[usize::from(v0)][usize::from(which_pos[y * width + x])] = d;
            }

            for (y2, x2) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if seen[y2 * width + x2] != v0 && grid[y2][x2] != b'#' {
                    seen[y2 * width + x2] = v0;
                    queue.push_back((y2, x2, d + 1));
                }
            }
        }
    }

    let mut dp = vec![vec![usize::MAX / 2; 1 << (num_pos - 1)]; usize::from(num_pos) - 1];
    for bs in 1_usize..1 << (num_pos - 1) {
        if bs.is_power_of_two() {
            dp[0][bs] = dist[0][bs.trailing_zeros() as usize + 1];
            continue;
        }

        for last in bits(bs) {
            for prev in bits(bs & !(1 << last)) {
                dp[last][bs] =
                    dp[last][bs].min(dp[prev][bs & !(1 << last)] + dist[prev + 1][last + 1]);
            }
        }
    }

    let all = (1 << (num_pos - 1)) - 1;
    (0..usize::from(num_pos) - 1)
        .map(|i| (dp[i][all], dp[i][all] + dist[i + 1][0]))
        .fold((usize::MAX, usize::MAX), |(a, b), (c, d)| {
            (a.min(c), b.min(d))
        })
}
