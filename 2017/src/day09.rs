use anyhow::{Result, ensure};
use register::register;

#[register]
fn run(input: &str) -> Result<(usize, usize)> {
    let mut depth = 1;
    let mut in_garbage = false;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut chars = input.bytes();
    while let Some(c) = chars.next() {
        if in_garbage {
            match c {
                b'>' => {
                    in_garbage = false;
                }
                b'!' => {
                    chars.next();
                }
                _ => {
                    part2 += 1;
                }
            }
        } else {
            match c {
                b'{' => {
                    part1 += depth;
                    depth += 1;
                }
                b'}' => {
                    depth -= 1;
                }
                b'<' => {
                    in_garbage = true;
                }
                _ => {}
            }
        }
    }

    ensure!(depth == 1, "unmatched groups in input");
    Ok((part1, part2))
}
