use anyhow::Result;
use utils::md5::Stack;

pub fn run(input: &str) -> Result<(String, usize)> {
    let mut part1 = None::<String>;
    let mut part2 = 0;
    let mut stack = Stack::new();
    stack.push_slice(input.as_bytes());
    dfs(0, 0, &mut stack, &mut |stack| {
        let len = stack.bytes().len() - input.len();
        if len < part1.as_ref().map_or(usize::MAX, |s| s.len()) {
            part1 = Some(String::from_utf8(stack.bytes()[input.len()..].to_vec()).unwrap());
        }
        part2 = part2.max(len);
    });

    Ok((part1.expect("no solution found"), part2))
}

fn dfs(x: u8, y: u8, md5_stack: &mut Stack, callback: &mut impl FnMut(&Stack)) {
    if x == 3 && y == 3 {
        callback(md5_stack);
        return;
    }

    let [up_down, left_right, ..] = md5_stack.digest()[0].to_be_bytes();
    if y > 0 && up_down >> 4 > 0xA {
        md5_stack.push(b'U');
        dfs(x, y - 1, md5_stack, callback);
        md5_stack.pop();
    }
    if y < 3 && up_down & 0xF > 0xA {
        md5_stack.push(b'D');
        dfs(x, y + 1, md5_stack, callback);
        md5_stack.pop();
    }
    if x > 0 && left_right >> 4 > 0xA {
        md5_stack.push(b'L');
        dfs(x - 1, y, md5_stack, callback);
        md5_stack.pop();
    }
    if x < 3 && left_right & 0xF > 0xA {
        md5_stack.push(b'R');
        dfs(x + 1, y, md5_stack, callback);
        md5_stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            ("ihgpwlah", ("DDRRRD", 370)),
            ("kglvqrro", ("DDUDRLRRUDRD", 492)),
            ("ulqzkmiv", ("DRURDRUDDLLDLUURRDULRLDUUDDDRR", 830)),
        ];
        for (input, (expected1, expected2)) in cases {
            let (actual1, actual2) = run(input).unwrap();
            assert_eq!(actual1, expected1);
            assert_eq!(actual2, expected2);
        }
    }
}
