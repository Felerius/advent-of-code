use anyhow::Result;
use arrayvec::ArrayString;
use utils::{input, ocr};

pub(crate) fn run(input: &str) -> Result<(usize, ArrayString<10>)> {
    let mut grid = [[false; 50]; 6];
    for line in input.lines() {
        let [a, b] = input::integers(line);
        if line.starts_with("rect") {
            for row in &mut grid[..b] {
                row[..a].fill(true);
            }
        } else if line[7..].starts_with("row") {
            grid[a].rotate_right(b);
        } else {
            let mut column = [false; 6];
            for i in 0..6 {
                column[i] = grid[i][a];
            }
            column.rotate_right(b);
            for i in 0..6 {
                grid[i][a] = column[i];
            }
        }
    }

    let part1 = grid.iter().flatten().filter(|&&x| x).count();
    let mut part2 = ArrayString::new();
    for i in 0..10 {
        let bits = (0..6)
            .flat_map(|y| &grid[y][5 * i..5 * (i + 1)])
            .fold(0, |bits, &x| bits << 1 | u32::from(x));
        part2.push(char::from(ocr::character(bits)?));
    }

    Ok((part1, part2))
}
