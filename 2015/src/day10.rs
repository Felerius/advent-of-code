use std::mem;

pub fn run(input: &str) -> (usize, usize) {
    let mut current: Vec<_> = input.bytes().map(|c| c - b'0').collect();
    let mut next = Vec::new();

    for _ in 0..40 {
        step(&current, &mut next);
        mem::swap(&mut current, &mut next);
        next.clear();
    }
    let part1 = current.len();

    for _ in 0..10 {
        step(&current, &mut next);
        mem::swap(&mut current, &mut next);
        next.clear();
    }
    let part2 = current.len();

    (part1, part2)
}

fn step(input: &[u8], output: &mut Vec<u8>) {
    let mut i = 0;
    while let Some(&c) = input.get(i) {
        let mut count = input[i..].iter().take_while(|&&c2| c2 == c).count();
        i += count;

        let mut digits = 0;
        while count > 0 {
            digits += 1;
            output.push((count % 10) as u8);
            count /= 10;
        }
        let start = output.len() - digits;
        output[start..].reverse();
        output.push(c);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn step() {
        let inputs = [
            ("1", "11"),
            ("11", "21"),
            ("21", "1211"),
            ("1211", "111221"),
        ];
        for (input, expected) in inputs {
            let input: Vec<_> = input.bytes().map(|c| c - b'0').collect();
            let expected: Vec<_> = expected.bytes().map(|c| c - b'0').collect();
            let mut actual = Vec::new();
            super::step(&input, &mut actual);
            assert_eq!(actual, expected, "failed for {input:?}");
        }
    }
}
