use anyhow::Result;
use itertools::Itertools;
use num::Integer;
use register::register;
use utils::input::Input;

#[register]
fn run(input: &str) -> Result<(usize, u16)> {
    run_parameterized(input, 101, 103)
}

fn run_parameterized(input: &str, width: u16, height: u16) -> Result<(usize, u16)> {
    assert!(width % 2 == 1 && height % 2 == 1);
    let robots: Vec<_> = input
        .lines()
        .map(|line| {
            let [x, y, dx, dy] = line.signed_integers_n::<i16, 4>()?;
            let dx = dx.rem_euclid(width as i16) as u16;
            let dy = dy.rem_euclid(height as i16) as u16;
            anyhow::Ok(Robot {
                x: x as u16,
                y: y as u16,
                dx,
                dy,
            })
        })
        .try_collect()?;

    let [q1, q2, q3, q4] = robots.iter().fold([0; 4], |mut quadrants, robot| {
        let x = robot.x(100, width);
        let y = robot.y(100, height);
        if x != width / 2 && y != height / 2 {
            let h2 = height.div_ceil(2);
            let w2 = width.div_ceil(2);
            let quadrant = 2 * (y / h2) + x / w2;
            quadrants[usize::from(quadrant)] += 1;
        }

        quadrants
    });
    let part1 = q1 * q2 * q3 * q4;

    let tree_x = find_min_variance(&robots, width, |robot, time| robot.x(time, width));
    let tree_y = find_min_variance(&robots, height, |robot, time| robot.y(time, height));
    let [tree_x, tree_y, width, height] = [tree_x, tree_y, width, height].map(i32::from);
    let gcd = width.extended_gcd(&height);
    assert_eq!(gcd.gcd, 1);
    let part2 = (tree_y - tree_x) % height * gcd.x % height * width + tree_x;
    let part2 = part2.rem_euclid(width * height) as u16;

    Ok((part1, part2))
}

fn find_min_variance<'a>(
    robots: &'a [Robot],
    dim: u16,
    mut coord: impl FnMut(&'a Robot, u16) -> u16,
) -> u16 {
    (0..dim)
        .min_by_key(|&time| {
            let coord_sum: u32 = robots
                .iter()
                .map(|robot| u32::from(coord(robot, time)))
                .sum();
            let rounded_mean = coord_sum / robots.len() as u32;

            robots
                .iter()
                .map(|robot| {
                    let x = coord(robot, time);
                    rounded_mean.abs_diff(u32::from(x)).pow(2)
                })
                .sum::<u32>()
        })
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: u16,
    y: u16,
    dx: u16,
    dy: u16,
}

impl Robot {
    fn x(self, time: u16, width: u16) -> u16 {
        (self.x + time * self.dx).rem_euclid(width)
    }

    fn y(self, time: u16, height: u16) -> u16 {
        (self.y + time * self.dy).rem_euclid(height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1() {
        assert_eq!(run_parameterized(INPUT, 11, 7).unwrap().0, 12);
    }
}
