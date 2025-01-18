use anyhow::{bail, Context, Result};

const MAX_WIRE: usize = 26 * 27;

pub(crate) fn run(input: &str) -> Result<(u16, u16)> {
    let mut instr = [Instruction::Assign(Input::Literal(0)); MAX_WIRE];
    for line in input.lines() {
        let (head, tail) = line.split_once(" -> ").context("invalid input")?;
        let wire = parse_wire(tail)?;
        instr[usize::from(wire)] = parse_instruction(head)?;
    }

    let mut cache = [None; MAX_WIRE];
    let part1 = eval(0, &instr, &mut cache);
    cache.fill(None);
    cache[1] = Some(part1);
    let part2 = eval(0, &instr, &mut cache);

    Ok((part1, part2))
}

#[derive(Debug, Copy, Clone)]
enum Input {
    Literal(u16),
    Wire(u16),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Assign(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, Input),
    RShift(Input, Input),
    Not(Input),
}

fn eval(wire: usize, instr: &[Instruction; MAX_WIRE], cache: &mut [Option<u16>; MAX_WIRE]) -> u16 {
    if let Some(x) = cache[wire] {
        return x;
    }

    let x = match instr[wire] {
        Instruction::Assign(input) => eval_input(input, instr, cache),
        Instruction::And(left, right) => {
            eval_input(left, instr, cache) & eval_input(right, instr, cache)
        }
        Instruction::Or(left, right) => {
            eval_input(left, instr, cache) | eval_input(right, instr, cache)
        }
        Instruction::LShift(left, right) => {
            eval_input(left, instr, cache) << eval_input(right, instr, cache)
        }
        Instruction::RShift(left, right) => {
            eval_input(left, instr, cache) >> eval_input(right, instr, cache)
        }
        Instruction::Not(input) => !eval_input(input, instr, cache),
    };
    cache[wire] = Some(x);
    x
}

fn eval_input(
    input: Input,
    instr: &[Instruction; MAX_WIRE],
    cache: &mut [Option<u16>; MAX_WIRE],
) -> u16 {
    match input {
        Input::Literal(x) => x,
        Input::Wire(wire) => eval(usize::from(wire), instr, cache),
    }
}

fn parse_instruction(s: &str) -> Result<Instruction> {
    let Some((head, tail)) = s.split_once(" ") else {
        return Ok(Instruction::Assign(parse_input(s)?));
    };
    let Some((mid, tail)) = tail.split_once(" ") else {
        debug_assert_eq!(head, "NOT");
        return Ok(Instruction::Not(parse_input(tail)?));
    };
    let left = parse_input(head)?;
    let right = parse_input(tail)?;
    let instr = match mid {
        "AND" => Instruction::And(left, right),
        "OR" => Instruction::Or(left, right),
        "LSHIFT" => Instruction::LShift(left, right),
        "RSHIFT" => Instruction::RShift(left, right),
        _ => bail!("Invalid instruction: {mid}"),
    };
    Ok(instr)
}

fn parse_input(s: &str) -> Result<Input> {
    if s.as_bytes()[0].is_ascii_digit() {
        Ok(Input::Literal(s.parse()?))
    } else {
        parse_wire(s).map(Input::Wire)
    }
}

fn parse_wire(s: &str) -> Result<u16> {
    let wire = match s.as_bytes() {
        [x] => u16::from(x - b'a'),
        [x, y] => u16::from(x - b'a' + 1) * 26 + u16::from(y - b'a'),
        _ => bail!("Invalid wire: {s}"),
    };
    Ok(wire)
}
