use std::fmt::{self, Display, Formatter};

use utils::hash::{FastHashCollectionExt, FastHashSet};

const NAMES: [(&str, &str); 5] = [
    ("promethium generator", "promethium-compatible microchip"),
    ("curium generator", "curium-compatible microchip"),
    ("ruthenium generator", "ruthenium-compatible microchip"),
    ("cobalt generator", "cobalt-compatible microchip"),
    ("plutonium generator", "plutonium-compatible microchip"),
];

pub(crate) fn run(input: &str) -> (usize, usize) {
    let mut initial_state1 = State {
        floors: [(0, 0); 4],
        elevator: 0,
    };
    for (i, line) in input.lines().enumerate() {
        for (j, (gen_name, chip_name)) in NAMES.into_iter().enumerate() {
            if line.contains(gen_name) {
                initial_state1.floors[i].0 |= 1 << j;
            }
            if line.contains(chip_name) {
                initial_state1.floors[i].1 |= 1 << j;
            }
        }
    }
    let part1 = find_shortest_solution::<5, 35>(initial_state1);

    let mut initial_state2 = initial_state1;
    initial_state2.floors[0].0 |= 0b110_0000;
    initial_state2.floors[0].1 |= 0b110_0000;
    let part2 = find_shortest_solution::<7, 63>(initial_state2);

    (part1, part2)
}
fn find_shortest_solution<const N: usize, const M: usize>(initial_state: State) -> usize {
    let all_bits = u8::MAX >> (8 - N);
    let target_state = State {
        floors: [(0, 0), (0, 0), (0, 0), (all_bits, all_bits)],
        elevator: 3,
    };
    debug_assert_eq!(0, target_state.move_lower_bound());
    let possible_moves = State::possible_moves::<N, M>();

    let mut seen = FastHashSet::with_capacity(1 << 14);
    let mut cur = vec![initial_state];
    let mut next = Vec::new();
    seen.insert(initial_state.to_equivalence());

    for dist in 1.. {
        for state in cur.drain(..) {
            for next_state in state.moves(possible_moves) {
                if next_state == target_state {
                    return dist;
                } else if seen.insert(next_state.to_equivalence()) {
                    next.push(next_state);
                }
            }
        }

        std::mem::swap(&mut cur, &mut next);
    }

    unreachable!("infinite loop terminated")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    floors: [(u8, u8); 4],
    elevator: u8,
}

impl State {
    fn is_valid(self) -> bool {
        self.floors.into_iter().all(|(gens, chips)| {
            let chips_without = chips & !gens;
            chips_without == 0 || gens == 0
        })
    }

    fn to_equivalence(self) -> StateEquivalence {
        let floors = self
            .floors
            .map(|(gens, chips)| (gens.count_ones() as u8) << 4 | chips.count_ones() as u8);
        StateEquivalence {
            floors,
            elevator: self.elevator,
        }
    }

    fn move_lower_bound(self) -> usize {
        let dist: usize = self
            .floors
            .into_iter()
            .enumerate()
            .map(|(i, (gens, chips))| {
                (3 - i) * (gens.count_ones() as usize + chips.count_ones() as usize)
            })
            .sum();
        dist.div_ceil(2)
    }

    fn moves<const M: usize>(self, possible_moves: [(u8, u8); M]) -> impl Iterator<Item = Self> {
        let from = usize::from(self.elevator);
        let (cur_gens, cur_chips) = self.floors[from];

        possible_moves
            .into_iter()
            .filter(move |&(gen_mask, chip_mask)| {
                gen_mask & cur_gens == gen_mask && chip_mask & cur_chips == chip_mask
            })
            .flat_map(move |(gen_mask, chip_mask)| {
                let target_floors = itertools::chain!(
                    self.elevator.checked_sub(1),
                    (self.elevator < 3).then_some(self.elevator + 1)
                );
                target_floors.map(move |to| {
                    let mut state = Self {
                        elevator: to,
                        ..self
                    };
                    state.floors[from].0 &= !gen_mask;
                    state.floors[from].1 &= !chip_mask;
                    state.floors[usize::from(to)].0 |= gen_mask;
                    state.floors[usize::from(to)].1 |= chip_mask;
                    state
                })
            })
            .filter(|state| state.is_valid())
    }

    fn possible_moves<const N: usize, const M: usize>() -> [(u8, u8); M] {
        let mut moves = [(0, 0); M];
        let mut idx = 0;
        for i in 0..N {
            moves[idx] = (1 << i, 0);
            moves[idx + 1] = (0, 1 << i);
            moves[idx + 2] = (1 << i, 1 << i);
            idx += 3;

            for j in i + 1..N {
                moves[idx] = (1 << i | 1 << j, 0);
                moves[idx + 1] = (0, 1 << i | 1 << j);
                idx += 2;
            }
        }

        moves
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let n = self
            .floors
            .into_iter()
            .map(|(gens, chips)| 8 - (gens | chips).leading_zeros())
            .max()
            .unwrap();

        for i in (0..4).rev() {
            write!(f, "F{} ", i + 1)?;
            if i == usize::from(self.elevator) {
                write!(f, "E  ")?;
            } else {
                write!(f, "   ")?;
            }

            let (gens, chips) = self.floors[i];
            for j in 0..n {
                if (gens >> j) & 1 == 1 {
                    write!(f, "{j}G ")?;
                } else {
                    write!(f, "   ")?;
                }
                if (chips >> j) & 1 == 1 {
                    write!(f, "{j}M ")?;
                } else {
                    write!(f, "   ")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StateEquivalence {
    floors: [u8; 4],
    elevator: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let initial_state = State {
            floors: [(0b00, 0b11), (0b01, 0b00), (0b10, 0b00), (0b00, 0b00)],
            elevator: 0,
        };
        let actual = find_shortest_solution::<2, 8>(initial_state);
        assert_eq!(11, actual);
    }
}
