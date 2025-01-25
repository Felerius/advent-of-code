use utils::input;

pub(crate) fn run(input: &str) -> (u32, u32) {
    let mut lines = input.lines();
    let [a] = input::integers::<u32, 1>(lines.nth(19).unwrap());
    let [b] = input::integers::<u32, 1>(lines.next().unwrap());
    (5040 + a * b, 479_001_600 + a * b)
}
