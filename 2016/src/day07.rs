use anyhow::Result;

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let mut abas = [[[usize::MAX; 26]; 26]; 2];
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut suffix = [b' '; 3];
        let mut in_bracket = false;
        let mut is_part1 = [false, false];
        let mut is_part2 = false;
        for c in line.bytes() {
            match c {
                b'[' => {
                    in_bracket = true;
                    suffix = [b' '; 3];
                }
                b']' => {
                    in_bracket = false;
                    suffix = [b' '; 3];
                }
                _ => {
                    if suffix[1] == c {
                        let a = usize::from(suffix[1] - b'a');
                        let b = usize::from(suffix[2] - b'a');
                        abas[usize::from(in_bracket)][a][b] = i;
                        is_part2 |= abas[usize::from(!in_bracket)][b][a] == i;
                    }
                    let is_abba =
                        suffix[0] == c && suffix[1] == suffix[2] && suffix[0] != suffix[1];
                    is_part1[usize::from(in_bracket)] |= is_abba;
                    suffix = [suffix[1], suffix[2], c];
                }
            }
        }

        part1 += usize::from(is_part1[0] && !is_part1[1]);
        part2 += usize::from(is_part2);
    }

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn";

    #[test]
    fn part1() {
        assert_eq!(2, run(INPUT).unwrap().0);
    }
}
