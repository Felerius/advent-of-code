use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Result;
use arrayvec::ArrayVec;
use register::register;
use utils::{hash::FastHashMap, input::Input};

#[register]
fn run(input: &str) -> Result<(u16, u16)> {
    let [boss_hp, boss_dmg] = input.unsigned_integers_n()?;
    Ok((solve(boss_hp, boss_dmg, 0), solve(boss_hp, boss_dmg, 1)))
}

#[allow(clippy::similar_names)]
fn solve(boss_hp: u8, boss_dmg: u8, hp_drain: u8) -> u16 {
    let initial = State {
        mana: 500,
        hp: 50,
        boss_hp,
        shield_turns: 0,
        poison_turns: 0,
        recharge_turns: 0,
    };
    let mut queue = BinaryHeap::from_iter([(Reverse(0), initial)]);
    let mut seen = FastHashMap::from_iter([(initial, 0)]);
    let mut upper_bound = u16::MAX;
    while let Some((Reverse(mana_spent), mut state)) = queue.pop() {
        if mana_spent >= upper_bound {
            return upper_bound;
        }

        if state.boss_hp == 0 {
            return mana_spent;
        }
        if state.process_turn_start(hp_drain) {
            if state.hp > 0 {
                return mana_spent;
            }
            continue;
        }

        // If we kill the boss directly with the immediate damage from one of
        // our spells, it will always be magic missile as it does both more
        // damage and has lower cost than drain. Thus we can check it once here
        // and otherwise assume the boss dies to poison.
        if state.mana >= 53 && state.boss_hp <= 4 {
            upper_bound = upper_bound.min(mana_spent + 53);
            continue;
        }

        let mut states = ArrayVec::<_, 5>::new();
        if let Some(mana2) = state.mana.checked_sub(53) {
            let state2 = State {
                mana: mana2,
                boss_hp: state.boss_hp.saturating_sub(4),
                ..state
            };
            states.push((mana_spent + 53, state2));
        }
        if let Some(mana2) = state.mana.checked_sub(73) {
            let state2 = State {
                mana: mana2,
                hp: state.hp + 2,
                boss_hp: state.boss_hp.saturating_sub(2),
                ..state
            };
            states.push((mana_spent + 73, state2));
        }
        if state.mana >= 113 && state.shield_turns == 0 {
            let state2 = State {
                mana: state.mana - 113,
                shield_turns: 6,
                ..state
            };
            states.push((mana_spent + 113, state2));
        }
        if state.mana >= 173 && state.poison_turns == 0 {
            let state2 = State {
                mana: state.mana - 173,
                poison_turns: 6,
                ..state
            };
            states.push((mana_spent + 173, state2));
        }
        if state.mana >= 229 && state.recharge_turns == 0 {
            let state2 = State {
                mana: state.mana - 229,
                recharge_turns: 5,
                ..state
            };
            states.push((mana_spent + 229, state2));
        }

        for (mana_spent2, mut state2) in states {
            let ac = if state2.shield_turns > 0 { 7 } else { 0 };
            if state2.process_turn_start(0) {
                upper_bound = upper_bound.min(mana_spent2);
            } else {
                let dmg = boss_dmg.saturating_sub(ac).max(1);
                state2.hp = state2.hp.saturating_sub(dmg);
                if state2.hp > 0 {
                    let best_mana = seen.entry(state2).or_insert(u16::MAX);
                    if mana_spent2 < *best_mana {
                        *best_mana = mana_spent2;
                        queue.push((Reverse(mana_spent2), state2));
                    }
                }
            }
        }
    }

    unreachable!("no solution")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    mana: u16,
    hp: u8,
    boss_hp: u8,
    shield_turns: u8,
    poison_turns: u8,
    recharge_turns: u8,
}

impl State {
    fn process_turn_start(&mut self, hp_drain: u8) -> bool {
        self.hp = self.hp.saturating_sub(hp_drain);
        self.shield_turns = self.shield_turns.saturating_sub(1);
        if self.poison_turns > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison_turns -= 1;
        }
        if self.recharge_turns > 0 {
            self.mana += 101;
            self.recharge_turns -= 1;
        }

        self.hp == 0 || self.boss_hp == 0
    }
}
