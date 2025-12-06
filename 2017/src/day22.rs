use register::register;
use utils::{grid, hash::FastHashMap};

#[register]
fn run(input: &str) -> (usize, usize) {
    let grid = grid::from_lines(input);
    let start = (grid.nrows() as isize / 2, grid.ncols() as isize / 2);
    let (cells1, cells2) = grid
        .indexed_iter()
        .map(|((y, x), &c)| {
            let pos = (y as isize, x as isize);
            let (state1, state2) = if c == b'#' {
                (State1::Infected, State2::Infected)
            } else {
                (State1::Clean, State2::Clean)
            };
            ((pos, state1), (pos, state2))
        })
        .unzip();

    let part1 = simulate(cells1, start, 10_000);
    let part2 = simulate(cells2, start, 10_000_000);

    (part1, part2)
}

fn simulate<S: State>(
    mut cells: FastHashMap<(isize, isize), S>,
    mut pos: (isize, isize),
    bursts: usize,
) -> usize {
    let mut dir = Direction::Up;
    let mut infections = 0;

    for _ in 0..bursts {
        let state = cells.entry(pos).or_default();
        dir = state.turn(dir);
        *state = state.next();
        infections += usize::from(*state == S::INFECTED);
        pos = dir.apply(pos);
    }

    infections
}

trait State: Copy + Default + Eq {
    const INFECTED: Self;

    fn turn(self, dir: Direction) -> Direction;
    fn next(self) -> Self;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum State1 {
    #[default]
    Clean,
    Infected,
}

impl State for State1 {
    const INFECTED: Self = Self::Infected;

    fn turn(self, dir: Direction) -> Direction {
        match self {
            Self::Clean => dir.turn_left(),
            Self::Infected => dir.turn_right(),
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Clean => Self::Infected,
            Self::Infected => Self::Clean,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum State2 {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State for State2 {
    const INFECTED: Self = Self::Infected;

    fn turn(self, dir: Direction) -> Direction {
        match self {
            Self::Clean => dir.turn_left(),
            Self::Weakened => dir,
            Self::Infected => dir.turn_right(),
            Self::Flagged => dir.opposite(),
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn opposite(self) -> Self {
        self.turn_left().turn_left()
    }

    fn turn_right(self) -> Self {
        self.turn_left().turn_left().turn_left()
    }

    fn apply(self, (y, x): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (y - 1, x),
            Self::Right => (y, x + 1),
            Self::Down => (y + 1, x),
            Self::Left => (y, x - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
..#
#..
...";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (5587, 2_511_944));
    }
}
