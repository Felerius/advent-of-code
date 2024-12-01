use anyhow::{bail, Context, Result};
use utils::input;

pub fn run(input: &str) -> Result<(u8, u32)> {
    let mut state = State::default();
    for line in input.lines() {
        if line.starts_with("value") {
            let [val, bot] = input::integers(line);
            state.ensure(bot);
            state.give(Target::Bot(bot), val)?;
        } else {
            let [bot, low, high] = input::integers(line);
            let (_, tail) = line.split_once(" gives ").context("invalid instruction")?;
            let low_to_bot = tail[7..].starts_with("bot");
            let (_, tail) = tail.split_once(" and ").context("invalid instruction")?;
            let high_to_bot = tail[8..].starts_with("bot");
            state.ensure(bot);
            state.instructions[usize::from(bot)] =
                (Target::new(low, low_to_bot), Target::new(high, high_to_bot));
        }
    }

    let mut part1 = 0;
    while let Some(bot) = state.ready.pop() {
        let Some((val1, Some(val2))) = state.inputs[usize::from(bot)] else {
            panic!("invalid internal state");
        };
        let low = val1.min(val2);
        let high = val1.max(val2);
        if (low, high) == (17, 61) {
            part1 = bot;
        }

        let (low_target, high_target) = state.instructions[usize::from(bot)];
        state.give(low_target, low)?;
        state.give(high_target, high)?;
    }

    let part2 = state.outputs[..3].iter().map(|&x| u32::from(x)).product();
    Ok((part1, part2))
}

#[derive(Clone, Copy)]
enum Target {
    Bot(u8),
    Output(u8),
}

impl Target {
    fn new(to: u8, is_bot: bool) -> Self {
        if is_bot {
            Self::Bot(to)
        } else {
            Self::Output(to)
        }
    }
}

#[derive(Default)]
struct State {
    inputs: Vec<Option<(u8, Option<u8>)>>,
    instructions: Vec<(Target, Target)>,
    outputs: Vec<u8>,
    ready: Vec<u8>,
}

impl State {
    fn ensure(&mut self, bot: u8) {
        if usize::from(bot) >= self.inputs.len() {
            self.inputs.resize(usize::from(bot) + 1, None);
            let default = Target::Output(u8::MAX);
            self.instructions
                .resize(usize::from(bot) + 1, (default, default));
        }
    }

    fn give(&mut self, target: Target, val: u8) -> Result<()> {
        match target {
            Target::Bot(bot) => {
                self.inputs[usize::from(bot)] = match self.inputs[usize::from(bot)] {
                    None => Some((val, None)),
                    Some((x, None)) => {
                        self.ready.push(bot);
                        Some((x, Some(val)))
                    }
                    Some(_) => bail!("bot {bot} received more than 2 inputs"),
                };
            }
            Target::Output(num) => {
                let num = usize::from(num);
                if num >= self.outputs.len() {
                    self.outputs.resize(num + 1, 0);
                }
                self.outputs[num] = val;
            }
        }
        Ok(())
    }
}
