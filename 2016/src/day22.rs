use anyhow::Result;
use utils::input::Input;

const MAX: usize = 600;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let mut num_free = [0; MAX];
    let mut num_used = [0; MAX];
    let mut width = 0;
    let mut num_large = 0;
    let mut pos_empty = (0, 0);
    for line in input.lines().skip(2) {
        let [x, y, size, used, _, _] = line.unsigned_integers_n::<usize, 6>()?;
        width = width.max(x + 1);
        num_large += usize::from(size >= 100);
        if used == 0 {
            pos_empty = (x, y);
        }

        num_free[size - used] += 1;
        num_used[used] += 1;
    }

    for i in (0..MAX - 1).rev() {
        num_free[i] += num_free[i + 1];
    }
    let part1 = num_used
        .into_iter()
        .enumerate()
        .skip(1)
        .map(|(used, count)| count * num_free[used])
        .sum();

    // Shamelessly using the structure of the input
    let left_of_large = width - num_large - 1;
    assert!(left_of_large <= pos_empty.0);
    let part2 =
        // Move empty node to the left
        pos_empty.0 - left_of_large +
        // Move it up
        pos_empty.1 +
        // Move it to the right
        width - 2 - left_of_large +
        // Move target data to the left
        width - 1 +
        // Move empty node around target data to enable its moves
        4 * (width - 2);

    Ok((part1, part2))
}
