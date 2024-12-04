use anyhow::Result;

pub fn run(input: &str) -> Result<(usize, usize)> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let part1 = itertools::iproduct!(0..height, 0..width)
        .flat_map(|(y, x)| {
            [
                (x + 3 < width).then(|| [(y, x), (y, x + 1), (y, x + 2), (y, x + 3)]),
                (y + 3 < height).then(|| [(y, x), (y + 1, x), (y + 2, x), (y + 3, x)]),
                (x + 3 < width && y + 3 < height)
                    .then(|| [(y, x), (y + 1, x + 1), (y + 2, x + 2), (y + 3, x + 3)]),
                (x + 3 < width && y >= 3)
                    .then(|| [(y, x), (y - 1, x + 1), (y - 2, x + 2), (y - 3, x + 3)]),
            ]
        })
        .flatten()
        .filter(|pos| {
            let s = pos.map(|(y, x)| grid[y][x]);
            s == *b"XMAS" || s == *b"SAMX"
        })
        .count();

    let part2 = itertools::iproduct!(1..height - 1, 1..width - 1)
        .filter(|&(y, x)| {
            const VALID: [(u8, u8); 2] = [(b'M', b'S'), (b'S', b'M')];
            let diag1 = (grid[y - 1][x - 1], grid[y + 1][x + 1]);
            let diag2 = (grid[y + 1][x - 1], grid[y - 1][x + 1]);
            grid[y][x] == b'A' && VALID.contains(&diag1) && VALID.contains(&diag2)
        })
        .count();

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(run(input).unwrap(), (18, 9));
    }
}
