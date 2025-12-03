use std::iter;

use register::register;

#[register]
fn run(input: &str) -> (usize, usize) {
    (count_safe(input, 40), count_safe(input, 400_000))
}

fn count_safe(first_row: &str, num_rows: usize) -> usize {
    let width = first_row.len();
    assert!(width <= 128);

    let first_row = first_row
        .bytes()
        .enumerate()
        .fold(0_u128, |bs, (i, c)| bs | u128::from(c == b'^') << i);
    let mask = (1 << width) - 1;
    iter::successors(Some(first_row), |row| Some((row >> 1) ^ (row << 1) & mask))
        .take(num_rows)
        .map(|row| width - row.count_ones() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_safe("..^^.", 3), 6);
        assert_eq!(count_safe(".^^.^.^^^^", 10), 38);
    }
}
