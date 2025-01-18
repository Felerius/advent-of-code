use anyhow::Result;
use utils::input;

pub(crate) fn run(input: &str) -> Result<(u64, u64)> {
    let [row, col] = input::integers::<u32, 2>(input);
    Ok((solve(row - 1, col - 1), 0))
}

fn solve(row: u32, col: u32) -> u64 {
    let diag = row + col;
    let diag_start_index = diag * (diag + 1) / 2;
    let mut index = diag_start_index + col;

    const MOD: u64 = 33554393;
    let mut res = 1;
    let mut base = 252533;
    while index > 0 {
        if index & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        index >>= 1;
    }

    res * 20151125 % MOD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(0, 0), 20151125);
        assert_eq!(solve(1, 0), 31916031);
        assert_eq!(solve(0, 1), 18749137);
        assert_eq!(solve(2, 0), 16080970);
        assert_eq!(solve(1, 1), 21629792);
        assert_eq!(solve(0, 2), 17289845);
        assert_eq!(solve(5, 4), 1534922);
    }
}
