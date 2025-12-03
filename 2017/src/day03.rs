use anyhow::Result;
use register::register;

#[allow(clippy::unreadable_literal)]
const SPIRAL: &[usize] = &[
    1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747,
    806, 880, 931, 957, 1968, 2105, 2275, 2391, 2450, 5022, 5336, 5733, 6155, 6444, 6591, 13486,
    14267, 15252, 16295, 17008, 17370, 35487, 37402, 39835, 42452, 45220, 47108, 48065, 98098,
    103128, 109476, 116247, 123363, 128204, 130654, 266330, 279138, 295229, 312453, 330785, 349975,
    363010, 369601, 752688, 787032, 830037, 875851, 924406, 975079, 1009457, 1026827, 2089141,
    2179400, 2292124, 2411813, 2539320, 2674100, 2814493, 2909666, 2957731, 6013560, 6262851,
    6573553, 6902404, 7251490, 7619304, 8001525, 8260383, 8391037, 17048404,
];

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let n: usize = input.parse()?;
    let mut largest_square = n.isqrt();
    largest_square -= 1 - largest_square % 2;
    let in_leg = (n - largest_square.pow(2)) % (largest_square + 1);
    let part1 = largest_square / 2 + 1 + in_leg.abs_diff(largest_square / 2 + 1);

    let i = SPIRAL.partition_point(|&x| x <= n);
    let part2 = SPIRAL[i];

    Ok((part1, part2))
}
