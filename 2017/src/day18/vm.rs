#![allow(dead_code, reason = "alternative implementation")]
use std::{
    collections::VecDeque,
    mem,
    ops::{ControlFlow, Index, IndexMut},
    str::FromStr,
};

use anyhow::{Context, Error, Result, bail, ensure};
use itertools::Itertools;

pub(crate) fn run(input: &str) -> Result<(Integer, usize)> {
    let program: Vec<Operation> = input.lines().map(str::parse).try_collect()?;

    let mut vm = VirtualMachine::new(&program);
    let mut ops = Part1Ops(0);
    vm.run(&mut ops);
    let Part1Ops(part1) = ops;

    let mut vms = [VirtualMachine::new(&program), VirtualMachine::new(&program)];
    vms[0].registers[Register(b'p' - b'a')] = 0;
    vms[1].registers[Register(b'p' - b'a')] = 1;
    let mut send_counts = [0, 0];
    let mut idx = 0;
    let mut ops = Part2Ops::default();
    for i in 0.. {
        if i > 1 && ops.recv.is_empty() {
            break;
        }

        vms[idx].run(&mut ops);
        send_counts[idx] += ops.send.len();
        mem::swap(&mut ops.send, &mut ops.recv);
        idx = 1 - idx;
    }

    Ok((part1, send_counts[1]))
}

struct Part1Ops(Integer);

impl VmOps for Part1Ops {
    fn snd(&mut self, value: Integer) {
        self.0 = value;
    }

    fn rcv(&mut self, value: Integer) -> ControlFlow<(), Option<Integer>> {
        if value != 0 {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(None)
        }
    }
}

#[derive(Default)]
struct Part2Ops {
    send: VecDeque<Integer>,
    recv: VecDeque<Integer>,
}

impl VmOps for Part2Ops {
    fn snd(&mut self, value: Integer) {
        self.send.push_back(value);
    }

    fn rcv(&mut self, _value: Integer) -> ControlFlow<(), Option<Integer>> {
        match self.recv.pop_front() {
            Some(v) => ControlFlow::Continue(Some(v)),
            None => ControlFlow::Break(()),
        }
    }
}

struct VirtualMachine<'a> {
    registers: Registers,
    program: &'a [Operation],
    pc: usize,
}

impl<'a> VirtualMachine<'a> {
    fn new(program: &'a [Operation]) -> Self {
        Self {
            registers: Registers::default(),
            program,
            pc: 0,
        }
    }

    fn run<V: VmOps>(&mut self, ops: &mut V) {
        loop {
            let mut new_pc = self.pc + 1;
            match self.program[self.pc] {
                Operation::Set(register, value) => {
                    self.registers[register] = value.eval(&self.registers);
                }
                Operation::Add(register, value) => {
                    self.registers[register] += value.eval(&self.registers);
                }
                Operation::Mul(register, value) => {
                    self.registers[register] *= value.eval(&self.registers);
                }
                Operation::Mod(register, value) => {
                    self.registers[register] %= value.eval(&self.registers);
                }
                Operation::Jgz(value1, value2) => {
                    if value1.eval(&self.registers) > 0 {
                        let offset = value2.eval(&self.registers);
                        new_pc = self.pc.strict_add_signed(offset as isize);
                    }
                }
                Operation::Snd(value) => ops.snd(value.eval(&self.registers)),
                Operation::Rcv(register) => match ops.rcv(self.registers[register]) {
                    ControlFlow::Continue(None) => {}
                    ControlFlow::Continue(Some(value)) => self.registers[register] = value,
                    ControlFlow::Break(()) => break,
                },
            }

            self.pc = new_pc;
        }
    }
}

#[derive(Default)]
struct Registers([Integer; 26]);

impl Index<Register> for Registers {
    type Output = Integer;

    fn index(&self, register: Register) -> &Self::Output {
        &self.0[usize::from(register.0)]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self.0[usize::from(register.0)]
    }
}

trait VmOps {
    fn snd(&mut self, value: Integer);
    fn rcv(&mut self, value: Integer) -> ControlFlow<(), Option<Integer>>;
}

type Integer = i64;

#[derive(Clone, Copy)]
struct Register(u8);

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        ensure!(bytes.len() == 1);
        ensure!(bytes[0].is_ascii_lowercase());
        Ok(Self(bytes[0] - b'a'))
    }
}

#[derive(Clone, Copy)]
enum Value {
    Immediate(Integer),
    Register(Register),
}

impl Value {
    fn eval(self, registers: &Registers) -> Integer {
        match self {
            Self::Immediate(v) => v,
            Self::Register(r) => registers[r],
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 && s.as_bytes()[0].is_ascii_lowercase() {
            s.parse().map(Self::Register)
        } else {
            Ok(Self::Immediate(s.parse()?))
        }
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Jgz(Value, Value),
    Snd(Value),
    Rcv(Register),
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = &s.as_bytes()[..3];
        let tail = &s[4..];
        if code == b"snd" {
            Ok(Self::Snd(tail.parse()?))
        } else if code == b"rcv" {
            Ok(Self::Rcv(tail.parse()?))
        } else {
            let (op1, op2) = tail.split_once(' ').context("expected two operands")?;
            match code {
                b"set" => Ok(Self::Set(op1.parse()?, op2.parse()?)),
                b"add" => Ok(Self::Add(op1.parse()?, op2.parse()?)),
                b"mul" => Ok(Self::Mul(op1.parse()?, op2.parse()?)),
                b"mod" => Ok(Self::Mod(op1.parse()?, op2.parse()?)),
                b"jgz" => Ok(Self::Jgz(op1.parse()?, op2.parse()?)),
                _ => bail!("unknown operation"),
            }
        }
    }
}
