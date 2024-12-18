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
    let mut grid = [[false; WIDTH]; HEIGHT];
    for &[x, y] in &bytes[..1024] {
        grid[y][x] = true;
    }

    let mut dist = [[usize::MAX; WIDTH]; HEIGHT];
    let mut queue = VecDeque::from_iter([(0, 0)]);
    dist[0][0] = 0;
    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == (WIDTH - 1, HEIGHT - 1) {
            break;
        }

        for (x2, y2) in neighbors(x, y) {
            if dist[y2][x2] == usize::MAX && !grid[y2][x2] {
                dist[y2][x2] = dist[y][x] + 1;
                queue.push_back((x2, y2));
            }
        }
    }
    let part1 = dist[HEIGHT - 1][WIDTH - 1];

    for &[x, y] in &bytes[1024..] {
        grid[y][x] = true;
    }
    let mut dsu = Dsu::new(WIDTH * HEIGHT);
    for (y, x) in itertools::iproduct!(0..HEIGHT, 0..WIDTH).filter(|&(y, x)| !grid[y][x]) {
        if x + 1 < WIDTH && !grid[y][x + 1] {
            dsu.merge(y * WIDTH + x, y * WIDTH + x + 1);
        }
        if y + 1 < HEIGHT && !grid[y + 1][x] {
            dsu.merge(y * WIDTH + x, (y + 1) * WIDTH + x);
        }
    }

    let [x, y] = bytes
        .iter()
        .rev()
        .find(|&&[x, y]| {
            grid[y][x] = false;
            for (x2, y2) in neighbors(x, y) {
                if !grid[y2][x2] {
                    dsu.merge(y * WIDTH + x, y2 * WIDTH + x2);
                }
            }

            dsu.find(0) == dsu.find(WIDTH * HEIGHT - 1)
        })
        .unwrap();
    let part2 = format!("{x},{y}");

    Ok((part1, part2))
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let neighbors = [
        x.checked_sub(1).map(|xi| (xi, y)),
        y.checked_sub(1).map(|yi| (x, yi)),
        (x + 1 < WIDTH).then(|| (x + 1, y)),
        (y + 1 < HEIGHT).then(|| (x, y + 1)),
    ];
    neighbors.into_iter().flatten()
}

struct Dsu(Vec<isize>);

impl Dsu {
    fn new(n: usize) -> Self {
        Self(vec![-1; n])
    }

    fn find(&mut self, i: usize) -> usize {
        if self.0[i] >= 0 {
            self.0[i] = self.find(self.0[i] as usize) as isize;
            self.0[i] as usize
        } else {
            i
        }
    }

    fn merge(&mut self, mut i: usize, mut j: usize) -> bool {
        i = self.find(i);
        j = self.find(j);
        if i == j {
            return false;
        }

        if self.0[j] < self.0[i] {
            (i, j) = (j, i);
        }

        self.0[i] += self.0[j];
        self.0[j] = i as isize;
        true
    }
}
