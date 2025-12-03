use anyhow::Result;
use itertools::Itertools;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    input
        .lines()
        .tuples()
        .try_fold((0, 0), |(part1, part2), (line1, line2, line3)| {
            let [a1, a2, a3] = line1.unsigned_integers_n()?;
            let [b1, b2, b3] = line2.unsigned_integers_n()?;
            let [c1, c2, c3] = line3.unsigned_integers_n()?;
            let p1 = [(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)];
            let p2 = [(a1, b1, c1), (a2, b2, c2), (a3, b3, c3)];
            Ok((part1 + count(p1), part2 + count(p2)))
        })
}

fn count(triples: [(u32, u32, u32); 3]) -> usize {
    let [a, b, c] = triples.map(|t| usize::from(check(t)));
    a + b + c
}

fn check((a, b, c): (u32, u32, u32)) -> bool {
    let mx = a.max(b).max(c);
    mx < a + b + c - mx
}
