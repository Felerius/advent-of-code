pub(crate) fn run(input: &str) -> (u32, u32) {
    let input = input.as_bytes();
    let n = input.len();

    input
        .iter()
        .enumerate()
        .fold((0, 0), |(mut part1, mut part2), (i, &c)| {
            if input[(i + 1) % n] == c {
                part1 += u32::from(c - b'0');
            }
            if input[(i + n / 2) % n] == c {
                part2 += u32::from(c - b'0');
            }

            (part1, part2)
        })
}
