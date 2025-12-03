use std::mem;

use itertools::Itertools;
use register::register;

#[register]
fn run(input: &str) -> (usize, usize) {
    let mut initial_grid = [[false; 102]; 102];
    for (i, lines) in input.lines().enumerate() {
        for (j, c) in lines.bytes().enumerate() {
            initial_grid[i + 1][j + 1] = c == b'#';
        }
    }

    let mut grid1 = Box::new(initial_grid);
    let mut grid2 = Box::new([[false; 102]; 102]);
    for _ in 0..100 {
        step(&grid1, &mut grid2, |_, _| false);
        mem::swap(&mut grid1, &mut grid2);
    }
    let part1 = grid1.iter().flatten().filter(|&&x| x).count();

    *grid1 = initial_grid;
    for _ in 0..100 {
        step(&grid1, &mut grid2, |i, j| {
            (i == 1 || i == 100) && (j == 1 || j == 100)
        });
        mem::swap(&mut grid1, &mut grid2);
    }
    let part2 = grid1.iter().flatten().filter(|&&x| x).count();

    (part1, part2)
}

fn step(
    from: &[[bool; 102]; 102],
    to: &mut [[bool; 102]; 102],
    mut locked_on: impl FnMut(usize, usize) -> bool,
) {
    for i in 1..101 {
        for j in 1..101 {
            to[i][j] = if locked_on(i, j) {
                true
            } else {
                let count = (i - 1..i + 2)
                    .cartesian_product(j - 1..j + 2)
                    .filter(|&(x, y)| from[x][y])
                    .count();
                if from[i][j] {
                    count == 3 || count == 4
                } else {
                    count == 3
                }
            };
        }
    }
}
