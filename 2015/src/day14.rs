use anyhow::{Context, Result};
use utils::input;

pub(crate) fn run(input: &str) -> Result<(u32, u32)> {
    let reindeers: Vec<[u32; 3]> = input.lines().map(|line| input::integers(line)).collect();

    let mut distance = vec![0; reindeers.len()];
    let mut points = vec![0; reindeers.len()];
    let mut max = 0;
    for i in 0..2503 {
        for (j, &[speed, duration, rest]) in reindeers.iter().enumerate() {
            if i % (duration + rest) < duration {
                distance[j] += speed;
                max = max.max(distance[j]);
            }
        }

        for j in 0..reindeers.len() {
            points[j] += u32::from(distance[j] == max);
        }
    }

    let part1 = distance.into_iter().max().context("empty input")?;
    let part2 = points.into_iter().max().context("empty input")?;
    Ok((part1, part2))
}
