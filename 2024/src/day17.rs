use itertools::Itertools;
use joinery::Joinable;
use utils::input;

pub fn run(input: &str) -> (String, u64) {
    run_testable(input, false)
}

fn run_testable(input: &str, skip_part2: bool) -> (String, u64) {
    let (l1, l2, l3, _, l5) = input.lines().collect_tuple().unwrap();
    let registers = [l1, l2, l3].map(|line| input::integers::<u64, 1>(line)[0]);
    let program: Vec<_> = l5[9..]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut part1_output = Vec::new();
    simulate(&program, registers, |i| {
        part1_output.push(i);
        true
    });
    let part1 = part1_output.join_with(',').to_string();

    let part2 = if skip_part2 {
        0
    } else {
        assert_eq!(registers[1], 0);
        assert_eq!(registers[2], 0);
        reconstruct_part2(&program, 0, 0).unwrap()
    };

    (part1, part2)
}

fn reconstruct_part2(program: &[u8], length: usize, value: u64) -> Option<u64> {
    if length == program.len() {
        return Some(value);
    }

    (0..8)
        .filter_map(|i| {
            let mut next = program.len() - 1 - length;
            let mut ok = true;
            let new_value = value << 3 | i;
            simulate(program, [new_value, 0, 0], |out| {
                ok &= program.get(next) == Some(&out);
                next += 1;
                ok
            });
            (ok && next == program.len()).then_some(new_value)
        })
        .find_map(|new_value| reconstruct_part2(program, length + 1, new_value))
}

fn simulate(program: &[u8], mut registers: [u64; 3], mut out: impl FnMut(u8) -> bool) {
    let mut ip = 0;
    loop {
        let Some(&opcode) = program.get(ip) else {
            break;
        };
        let Some(&operand) = program.get(ip + 1) else {
            break;
        };
        let combo = |registers: [u64; 3]| {
            if operand < 4 {
                u64::from(operand)
            } else {
                registers[usize::from(operand - 4)]
            }
        };

        let mut jumped = false;
        match opcode {
            0 => {
                registers[0] = registers[0]
                    .checked_shr(combo(registers) as u32)
                    .unwrap_or(0)
            }
            1 => registers[1] ^= u64::from(operand),
            2 => registers[1] = combo(registers) % 8,
            3 => {
                if registers[0] != 0 {
                    ip = usize::from(operand);
                    jumped = true;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => {
                if !out(combo(registers) as u8 % 8) {
                    break;
                }
            }
            6 => {
                registers[1] = registers[0]
                    .checked_shr(combo(registers) as u32)
                    .unwrap_or(0)
            }
            7 => {
                registers[2] = registers[0]
                    .checked_shr(combo(registers) as u32)
                    .unwrap_or(0)
            }
            _ => panic!("Invalid opcode: {opcode}"),
        }

        if !jumped {
            ip += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1() {
        assert_eq!(run_testable(INPUT1, true).0, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2() {
        assert_eq!(run(INPUT2).1, 117440);
    }
}
