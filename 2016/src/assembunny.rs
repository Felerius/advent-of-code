use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};

pub(crate) type Integer = u32;
pub(crate) type JumpOffset = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Register(u8);

impl Register {
    pub(crate) const A: Self = Self(0);
    pub(crate) const C: Self = Self(2);
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.as_bytes() {
            &[c] if c.is_ascii_lowercase() => Ok(Self(c - b'a')),
            _ => bail!("invalid register: {s:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Source {
    Immediate(Integer),
    Register(Register),
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(register) = s.parse() {
            Ok(Self::Register(register))
        } else {
            Ok(Self::Immediate(s.parse()?))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Instruction {
    Copy(Source, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Register, JumpOffset),
    Jump(JumpOffset),
    Noop,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let instr = s
            .as_bytes()
            .first_chunk::<3>()
            .context("instruction too short")?;
        let args = &s[4..];
        let res = match instr {
            b"cpy" => {
                let (a, b) = parse_two_args(args)?;
                Self::Copy(a, b)
            }
            b"inc" => Self::Increment(args.parse()?),
            b"dec" => Self::Decrement(args.parse()?),
            b"jnz" => {
                let (pred, off) = parse_two_args(args)?;
                match pred {
                    Source::Immediate(0) => Self::Noop,
                    Source::Immediate(_) => Self::Jump(off),
                    Source::Register(reg) => Self::JumpNotZero(reg, off),
                }
            }
            _ => bail!("invalid instruction: {instr:?}"),
        };
        Ok(res)
    }
}

fn parse_two_args<S, T>(s: &str) -> Result<(S, T)>
where
    S: FromStr,
    Error: From<<S as FromStr>::Err>,
    T: FromStr,
    Error: From<<T as FromStr>::Err>,
{
    let (a, b) = s.split_once(' ').context("expected two arguments")?;
    Ok((a.parse()?, b.parse()?))
}

#[derive(Debug, Clone)]
pub(crate) struct VirtualMachine<const N: usize> {
    registers: [Integer; N],
}

impl<const N: usize> VirtualMachine<N> {
    pub(crate) fn new() -> Self {
        Self { registers: [0; N] }
    }

    pub(crate) fn set(&mut self, register: Register, value: Integer) {
        self.registers[usize::from(register.0)] = value;
    }

    pub(crate) fn read(&self, register: Register) -> Integer {
        self.registers[usize::from(register.0)]
    }

    pub(crate) fn reset(&mut self) {
        self.registers = [0; N];
    }

    pub(crate) fn execute(&mut self, program: &[Instruction]) -> Result<Integer> {
        let mut pc = 0;
        while let Some(&instr) = program.get(pc) {
            let mut jump = None;
            match instr {
                Instruction::Copy(source, to) => {
                    let val = match source {
                        Source::Immediate(val) => val,
                        Source::Register(from) => self.read(from),
                    };
                    self.set(to, val);
                }
                Instruction::Increment(register) => self.set(register, self.read(register) + 1),
                Instruction::Decrement(register) => self.set(register, self.read(register) - 1),
                Instruction::JumpNotZero(register, offset) => {
                    jump = (self.read(register) != 0).then_some(offset);
                }
                Instruction::Jump(offset) => jump = Some(offset),
                Instruction::Noop => {}
            }

            pc = if let Some(offset) = jump {
                pc.checked_add_signed(isize::from(offset))
                    .context("program counter went below zero")?
            } else {
                pc + 1
            };
        }

        Ok(self.read(Register::A))
    }
}
