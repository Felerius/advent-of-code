use ndarray::Array2;
use register::register;
use utils::{
    grid,
    hash::{FastHashCollectionExt, FastHashMap},
};

#[register]
fn run(input: &str) -> (usize, usize) {
    let buffer = 200;
    let (height, width) = grid::from_lines(input).dim();
    let cells1 = Array2::from_elem((height + 2 * buffer, width + 2 * buffer), State1::default());
    let cells2 = Array2::from_elem((height + 2 * buffer, width + 2 * buffer), State2::default());
    let part1 = simulate::<State1>(input, cells1, buffer, 10_000);
    let part2 = simulate::<State2>(input, cells2, buffer, 10_000_000);
    (part1, part2)
}

#[register]
fn no_guessed_limits(input: &str) -> (usize, usize) {
    let offset = usize::MAX / 2;
    let part1 = simulate::<State1>(input, FastHashMap::new(), offset, 10_000);
    let part2 = simulate::<State2>(input, FastHashMap::new(), offset, 10_000_000);
    (part1, part2)
}

fn simulate<S: State>(input: &str, mut cells: impl Grid<S>, offset: usize, bursts: usize) -> usize {
    let grid = grid::from_lines(input);
    for ((y, x), &c) in grid.indexed_iter() {
        let cell = cells.get_mut((y + offset, x + offset));
        *cell = if c == b'#' { S::INFECTED } else { S::default() };
    }

    let mut pos = (grid.nrows() / 2 + offset, grid.ncols() / 2 + offset);
    let mut dir = Direction::Up;
    let mut infections = 0;
    for _ in 0..bursts {
        let state = cells.get_mut(pos);
        dir = state.turn(dir);
        *state = state.next();
        infections += usize::from(*state == S::INFECTED);
        pos = dir.apply(pos);
    }

    infections
}

trait Grid<T> {
    fn get_mut(&mut self, pos: (usize, usize)) -> &mut T;
}

impl<T> Grid<T> for Array2<T> {
    fn get_mut(&mut self, pos: (usize, usize)) -> &mut T {
        &mut self[pos]
    }
}

impl<T: Default> Grid<T> for FastHashMap<(usize, usize), T> {
    fn get_mut(&mut self, pos: (usize, usize)) -> &mut T {
        self.entry(pos).or_default()
    }
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

    fn apply(self, (y, x): (usize, usize)) -> (usize, usize) {
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
