use register::register;

#[register]
fn run(input: &str) -> (String, usize) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let on_path = |x: usize, y: usize| {
        grid.get(x)
            .and_then(|row| row.get(y))
            .is_some_and(|&c| c != b' ')
    };

    let mut x = 0;
    let mut y = grid[0].iter().position(|&c| c != b' ').unwrap();
    let mut dir = Direction::Down;
    let mut part1 = String::new();
    let mut part2 = 0;
    loop {
        part2 += 1;
        if grid[x][y].is_ascii_uppercase() {
            part1.push(char::from(grid[x][y]));
        }

        let (nx, ny) = dir.next(x, y);
        if on_path(nx, ny) {
            (x, y) = (nx, ny);
        } else {
            let Some((nx, ny, ndir)) = dir
                .turns(x, y)
                .into_iter()
                .find(|&(nx, ny, _)| on_path(nx, ny))
            else {
                break;
            };
            (x, y, dir) = (nx, ny, ndir);
        }
    }

    (part1, part2)
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::Up => (x.wrapping_sub(1), y),
            Self::Down => (x + 1, y),
            Self::Left => (x, y.wrapping_sub(1)),
            Self::Right => (x, y + 1),
        }
    }

    fn turns(self, x: usize, y: usize) -> [(usize, usize, Self); 2] {
        let dirs = match self {
            Self::Up | Self::Down => [Self::Left, Self::Right],
            Self::Left | Self::Right => [Self::Up, Self::Down],
        };
        dirs.map(|d| {
            let (nx, ny) = d.next(x, y);
            (nx, ny, d)
        })
    }
}
