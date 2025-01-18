use anyhow::{Context, Result};

pub(crate) fn run(input: &str) -> Result<(i64, i64)> {
    let (part1, part2, rest) = eval(input.as_bytes())?;
    debug_assert!(rest.is_empty());
    Ok((part1, part2))
}

fn eval(s: &[u8]) -> Result<(i64, i64, &[u8])> {
    let Some(&c0) = s.first() else {
        return Ok((0, 0, &[]));
    };

    match c0 {
        b'{' => eval_object(&s[1..]),
        b'[' => eval_array(&s[1..]),
        _ => eval_literal(s),
    }
}

fn eval_literal(mut s: &[u8]) -> Result<(i64, i64, &[u8])> {
    if s.starts_with(b"\"") {
        let len = s[1..]
            .iter()
            .position(|&c| c == b'"')
            .context("unterminated string")?;
        Ok((0, 0, &s[len + 2..]))
    } else {
        let mut num = 0;
        let negative = if s[0] == b'-' {
            s = &s[1..];
            true
        } else {
            false
        };
        while let Some(&c) = s.first().filter(|&&c| c.is_ascii_digit()) {
            num = num * 10 + i64::from(c - b'0');
            s = &s[1..];
        }
        let num = if negative { -num } else { num };
        Ok((num, num, s))
    }
}

fn eval_array(mut s: &[u8]) -> Result<(i64, i64, &[u8])> {
    let mut part1 = 0;
    let mut part2 = 0;
    while s[0] != b']' {
        let (val1, val2, rest) = eval(s)?;
        part1 += val1;
        part2 += val2;
        s = rest;

        if s[0] == b',' {
            s = &s[1..];
        }
    }

    Ok((part1, part2, &s[1..]))
}

fn eval_object(mut s: &[u8]) -> Result<(i64, i64, &[u8])> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut red = false;
    while s[0] != b'}' {
        let key_len = s[1..]
            .iter()
            .position(|&c| c == b'"')
            .context("unterminated string")?;
        s = &s[key_len + 3..];
        red |= s.starts_with(b"\"red\"");

        let (val1, val2, rest) = eval(s)?;
        part1 += val1;
        part2 += val2;
        s = rest;

        if s[0] == b',' {
            s = &s[1..];
        }
    }

    Ok((part1, if red { 0 } else { part2 }, &s[1..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let inputs = [
            ("[1,2,3]", 6),
            (r#"{"a":2,"b":4}"#, 6),
            ("[[[3]]]", 3),
            (r#"{"a":{"b":4},"c":-1}"#, 3),
            (r#"{"a":[-1,1]}"#, 0),
            (r#"[-1,{"a":1}]"#, 0),
            ("[]", 0),
            ("{}", 0),
        ];
        for (input, expected) in inputs {
            assert_eq!(run(input)?.0, expected, "failed for {input:?}");
        }
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let inputs = [
            ("[1,2,3]", 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ];
        for (input, expected) in inputs {
            assert_eq!(run(input)?.1, expected, "failed for {input:?}");
        }
        Ok(())
    }
}
