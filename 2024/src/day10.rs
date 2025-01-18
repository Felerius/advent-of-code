pub(crate) fn run(input: &str) -> (usize, usize) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut cache = vec![vec![(usize::MAX, 0); width]; height];
    itertools::iproduct!(0..height, 0..width)
        .filter(|&(y, x)| grid[y][x] == b'0')
        .map(|(y, x)| dfs(y, x, y * width + x, &grid, &mut cache))
        .fold((0, 0), |(a1, a2), (c1, c2)| (a1 + c1, a2 + c2))
}

fn dfs(
    y: usize,
    x: usize,
    idx: usize,
    grid: &[&[u8]],
    cache: &mut [Vec<(usize, usize)>],
) -> (usize, usize) {
    if cache[y][x].0 == idx {
        return (0, cache[y][x].1);
    }
    if grid[y][x] == b'9' {
        cache[y][x] = (idx, 1);
        return (1, 1);
    }

    let (cnt1, cnt2) = neighbors(y, x, grid)
        .map(|(y2, x2)| dfs(y2, x2, idx, grid, cache))
        .fold((0, 0), |(a1, a2), (c1, c2)| (a1 + c1, a2 + c2));

    cache[y][x] = (idx, cnt2);
    (cnt1, cnt2)
}

fn neighbors<'a>(
    y: usize,
    x: usize,
    grid: &'a [&'a [u8]],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let candidates = [
        y.checked_sub(1).map(|yi| (yi, x)),
        x.checked_sub(1).map(|xi| (y, xi)),
        (y + 1 < grid.len()).then(|| (y + 1, x)),
        (x + 1 < grid[0].len()).then(|| (y, x + 1)),
    ];

    candidates
        .into_iter()
        .flatten()
        .filter(move |&(y2, x2)| grid[y2][x2] == grid[y][x] + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test() {
        assert_eq!(run(INPUT), (36, 81));
    }
}
