use anyhow::Result;
use register::register;
use utils::input::Input;

const WEAPONS: &[(u32, u32)] = &[(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
const ARMORS: &[(u32, u32)] = &[(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];
const RING_COMBOS: &[(u32, u32, u32)] = &[
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (75, 3, 0),
    (125, 4, 0),
    (150, 5, 0),
    (20, 0, 1),
    (40, 0, 2),
    (60, 0, 3),
    (100, 0, 4),
    (120, 0, 5),
    (45, 1, 1),
    (65, 1, 2),
    (105, 1, 3),
    (70, 2, 1),
    (90, 2, 2),
    (130, 2, 3),
    (120, 3, 1),
    (140, 3, 2),
    (180, 3, 3),
];

#[register]
fn run(input: &str) -> Result<(u32, u32)> {
    let [boss_hp, boss_dmg, boss_ac] = input.unsigned_integers_n::<u32, 3>()?;
    let (part1, part2) = itertools::iproduct!(WEAPONS, ARMORS, RING_COMBOS)
        .map(|((c1, dmg1), (c2, ac1), (c3, dmg2, ac2))| {
            let my_dmg = dmg1 + dmg2;
            let my_ac = ac1 + ac2;
            let my_dpr = my_dmg.saturating_sub(boss_ac).max(1);
            let boss_dpr = boss_dmg.saturating_sub(my_ac).max(1);
            let win = 100_u32.div_ceil(boss_dpr) >= boss_hp.div_ceil(my_dpr);
            (win, c1 + c2 + c3)
        })
        .fold((u32::MAX, 0), |(mut part1, mut part2), (win, cost)| {
            if win {
                part1 = part1.min(cost);
            } else {
                part2 = part2.max(cost);
            }
            (part1, part2)
        });
    Ok((part1, part2))
}
