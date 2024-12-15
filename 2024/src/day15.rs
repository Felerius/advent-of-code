use std::fmt::{self, Display, Formatter};

use anyhow::Result;
use arrayvec::ArrayVec;
use itertools::Itertools;
use tinybitset::TinyBitSet;
use utils::hash::{FastHashCollectionExt, FastHashSet};

type RowBitSet1 = TinyBitSet<u64, 1>;
type RowBitSet2 = TinyBitSet<u128, 1>;
type RowArrayVec<T> = ArrayVec<T, 100>;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let mut lines = input.lines().map(|line| line.as_bytes());
    let mut robot = (0, 0);
    let mut width = 0;
    let (walls1, boxes1, grid2): (Vec<_>, Vec<_>, Vec<_>) = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            width = line.len();
            let mut walls1 = RowBitSet1::new();
            let mut boxes1 = RowBitSet1::new();
            let mut grid2 = RowArrayVec::new();
            for (x, &c) in line.iter().enumerate() {
                let grid_entries = match c {
                    b'#' => {
                        walls1.insert(x);
                        [Field::Wall, Field::Wall]
                    }
                    b'@' => {
                        robot = (x, y);
                        [Field::Empty, Field::Empty]
                    }
                    b'O' => {
                        boxes1.insert(x);
                        [Field::BoxLeft, Field::BoxRight]
                    }
                    _ => [Field::Empty, Field::Empty],
                };
                grid2.extend(grid_entries);
            }

            (walls1, boxes1, grid2)
        })
        .multiunzip();

    let mut state1 = State1 {
        robot,
        width,
        walls: walls1,
        boxes: boxes1,
    };
    let mut state2 = State2 {
        robot: (2 * robot.0, robot.1),
        grid: grid2,
    };

    for direction in lines.flatten() {
        let direction = match direction {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            _ => Direction::Right,
        };

        state1.process_move(direction);
        state2.process_move(direction);
    }

    Ok((state1.coordinate_sum(), state2.coordinate_sum()))
}

struct State1 {
    robot: (usize, usize),
    width: usize,
    walls: Vec<RowBitSet1>,
    boxes: Vec<RowBitSet1>,
}

impl State1 {
    fn process_move(&mut self, direction: Direction) {
        let robot_target = direction.offset(self.robot, 1);
        let end = (1..)
            .map(|i| direction.offset(self.robot, i))
            .find(|&(x, y)| self.walls[y][x] || !self.boxes[y][x])
            .unwrap();
        if !self.walls[end.1][end.0] {
            self.robot = robot_target;
            if end != robot_target {
                self.boxes[end.1].insert(end.0);
                self.boxes[self.robot.1].remove(self.robot.0);
            }
        }
    }

    fn coordinate_sum(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().map(move |x| 100 * y + x))
            .sum()
    }
}

impl Display for State1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (y, (walls, boxes)) in self.walls.iter().zip(&self.boxes).enumerate() {
            for x in 0..self.width {
                let c = if (x, y) == self.robot {
                    '@'
                } else if walls[x] {
                    '#'
                } else if boxes[x] {
                    'O'
                } else {
                    '.'
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct State2 {
    robot: (usize, usize),
    grid: Vec<RowArrayVec<Field>>,
}

impl State2 {
    fn process_move(&mut self, direction: Direction) {
        if matches!(direction, Direction::Left | Direction::Right) {
            let robot_target = direction.offset(self.robot, 1);
            let end = (1..)
                .map(|i| direction.offset(self.robot, i))
                .find(|&(x, y)| matches!(self.grid[y][x], Field::Wall | Field::Empty))
                .unwrap();
            if self.grid[end.1][end.0] == Field::Empty {
                if direction == Direction::Left {
                    self.grid[end.1][end.0..self.robot.0].rotate_left(1);
                } else {
                    self.grid[end.1][robot_target.0..=end.0].rotate_right(1);
                }
                self.robot = robot_target;
            }
            return;
        }

        let mut x_coords = RowBitSet2::from_iter([self.robot.0]);
        let mut boxes = FastHashSet::new();
        let y_coords = (1..).map(|i| direction.offset(self.robot, i).1);
        let mut blocked = false;
        'outer: for y in y_coords {
            let mut any_box = false;
            let mut new_x_coords = RowBitSet2::new();
            for x in x_coords {
                let field = self.grid[y][x];
                if field == Field::Wall {
                    blocked = true;
                    break 'outer;
                }

                if matches!(field, Field::BoxLeft | Field::BoxRight) {
                    any_box = true;
                    let left_x = if field == Field::BoxLeft { x } else { x - 1 };
                    boxes.insert((left_x, y));
                    new_x_coords.insert(left_x);
                    new_x_coords.insert(left_x + 1);
                }
            }

            x_coords = new_x_coords;
            if !any_box {
                break;
            }
        }

        if !blocked {
            for &(x, y) in &boxes {
                self.grid[y][x] = Field::Empty;
                self.grid[y][x + 1] = Field::Empty;
            }
            for &(x, y) in &boxes {
                let (x2, y2) = direction.offset((x, y), 1);
                self.grid[y2][x2] = Field::BoxLeft;
                self.grid[y2][x2 + 1] = Field::BoxRight;
            }
            self.robot = direction.offset(self.robot, 1);
        }
    }

    fn coordinate_sum(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &field)| field == Field::BoxLeft)
                    .map(move |(x, _)| 100 * y + x)
            })
            .sum()
    }
}

impl Display for State2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &field) in row.iter().enumerate() {
                let c = match field {
                    _ if (x, y) == self.robot => '@',
                    Field::Empty => '.',
                    Field::Wall => '#',
                    Field::BoxLeft => '[',
                    Field::BoxRight => ']',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(self, (x, y): (usize, usize), steps: usize) -> (usize, usize) {
        match self {
            Self::Up => (x, y - steps),
            Self::Down => (x, y + steps),
            Self::Left => (x - steps, y),
            Self::Right => (x + steps, y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const SMALL_INPUT2: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const LARGE_INPUT: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn small_part1() {
        let (part1, _) = run(SMALL_INPUT1).unwrap();
        assert_eq!(part1, 2028);
    }

    #[test]
    fn large_part1() {
        let (part1, _) = run(LARGE_INPUT).unwrap();
        assert_eq!(part1, 10092);
    }

    #[test]
    fn small_part2() {
        let (_, part2) = run(SMALL_INPUT2).unwrap();
        assert_eq!(part2, 105 + 207 + 306);
    }

    #[test]
    fn large_part2() {
        let (_, part2) = run(LARGE_INPUT).unwrap();
        assert_eq!(part2, 9021);
    }
}
