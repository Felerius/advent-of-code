use anyhow::Result;
use itertools::Itertools;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(i64, i64)> {
    let ingredients: Vec<[i32; 5]> = input.lines().map(Input::signed_integers_n).try_collect()?;
    let [ing1, ing2, ing3, ing4] = ingredients.try_into().expect("4 ingredients required");

    let mut part1 = 0;
    let mut part2 = 0;
    for a1 in 0..=100 {
        for a2 in 0..=(100 - a1) {
            for a3 in 0..=(100 - a1 - a2) {
                let a4 = 100 - a1 - a2 - a3;
                let score = (0..4)
                    .map(|i| {
                        let sum = a1 * ing1[i] + a2 * ing2[i] + a3 * ing3[i] + a4 * ing4[i];
                        i64::from(sum.max(0))
                    })
                    .product::<i64>()
                    .max(0);
                let calories = a1 * ing1[4] + a2 * ing2[4] + a3 * ing3[4] + a4 * ing4[4];
                part1 = part1.max(score);
                if calories == 500 {
                    part2 = part2.max(score);
                }
            }
        }
    }

    Ok((part1, part2))
}
