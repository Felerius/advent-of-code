use utils::input;

const MOD: u64 = 33_554_393;

pub(crate) fn run(input: &str) -> (u64, u8) {
    let [row, col] = input::integers::<u32, 2>(input);
    (solve(row - 1, col - 1), 0)
}

fn solve(row: u32, col: u32) -> u64 {
    let diag = row + col;
    let diag_start_index = diag * (diag + 1) / 2;
    let mut index = diag_start_index + col;

    let mut res = 1;
    let mut base = 252_533;
    while index > 0 {
        if index & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        index >>= 1;
    }

    res * 20_151_125 % MOD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(0, 0), 20_151_125);
        assert_eq!(solve(1, 0), 31_916_031);
        assert_eq!(solve(0, 1), 18_749_137);
        assert_eq!(solve(2, 0), 16_080_970);
        assert_eq!(solve(1, 1), 21_629_792);
        assert_eq!(solve(0, 2), 17_289_845);
        assert_eq!(solve(5, 4), 1_534_922);
    }
}
