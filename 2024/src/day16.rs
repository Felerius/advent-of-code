use std::{cmp::Reverse, collections::BinaryHeap, thread};

pub(crate) fn run(input: &str) -> (usize, usize) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut from = (usize::MAX, usize::MAX);
    let mut to = (usize::MAX, usize::MAX);
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                b'S' => from = (y, x),
                b'E' => to = (y, x),
                _ => {}
            }
        }
    }

    let (from_start, to_end) = thread::scope(|scope| {
        let from_start = scope.spawn(|| run_dijkstra(&grid, from, [Direction::East], to));
        let to_end = scope.spawn(|| run_dijkstra(&grid, to, Direction::ALL, from));
        (from_start.join().unwrap(), to_end.join().unwrap())
    });
    let part1 = from_start[to.0][to.1].into_iter().min().unwrap();
    let part2 = itertools::iproduct!(0..height, 0..width)
        .filter(|&(y, x)| {
            Direction::ALL.into_iter().any(|dir| {
                from_start[y][x][dir as usize] + to_end[y][x][dir.opposite() as usize] == part1
            })
        })
        .count();

    (part1, part2)
}

fn run_dijkstra(
    grid: &[&[u8]],
    from: (usize, usize),
    from_dirs: impl IntoIterator<Item = Direction>,
    to: (usize, usize),
) -> Vec<Vec<[usize; 4]>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut cheapest = vec![vec![[usize::MAX / 2; 4]; width]; height];
    let mut queue = BinaryHeap::new();
    for dir in from_dirs {
        cheapest[from.0][from.1][dir as usize] = 0;
        queue.push((Reverse(0), from, dir));
    }

    let mut upper_bound = usize::MAX;
    while let Some((Reverse(cost), pos, dir)) = queue.pop() {
        if pos == to {
            upper_bound = cost;
        }
        if cost > upper_bound {
            break;
        }
        if cheapest[pos.0][pos.1][dir as usize] < cost {
            continue;
        }

        let turns = dir.turn();
        let moves = [(pos, turns[0]), (pos, turns[1]), (dir.offset(pos), dir)];
        for (pos2, dir2) in moves {
            let cost2 = cost + if dir == dir2 { 1 } else { 1000 };
            if grid[pos2.0][pos2.1] != b'#' && cost2 < cheapest[pos2.0][pos2.1][dir2 as usize] {
                cheapest[pos2.0][pos2.1][dir2 as usize] = cost2;
                queue.push((Reverse(cost2), pos2, dir2));
            }
        }
    }

    cheapest
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    fn turn(self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::West, Self::East],
            Self::East | Self::West => [Self::North, Self::South],
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn offset(self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (y - 1, x),
            Self::East => (y, x + 1),
            Self::South => (y + 1, x),
            Self::West => (y, x - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_maze1() {
        assert_eq!(run(INPUT1).0, 7036);
    }

    #[test]
    fn part1_maze2() {
        assert_eq!(run(INPUT2).0, 11048);
    }

    #[test]
    fn part2_maze1() {
        assert_eq!(run(INPUT1).1, 45);
    }

    #[test]
    fn part2_maze2() {
        assert_eq!(run(INPUT2).1, 64);
    }
}
