use anyhow::Result;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::{
    hash::{FastHashCollectionExt, FastHashMap},
    input::Input,
};

pub(crate) fn run(input: &str) -> Result<(usize, usize)> {
    let particles: Vec<_> = input
        .lines()
        .map(|line| {
            let [x0, y0, z0, vx, vy, vz, ax, ay, az] = line.signed_integers_n()?;
            let cx = Coordinate(x0, vx, ax);
            let cy = Coordinate(y0, vy, ay);
            let cz = Coordinate(z0, vz, az);
            anyhow::Ok((cx, cy, cz))
        })
        .try_collect()?;

    let (part1, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, &(cx, cy, cz))| {
            let t = 10_000_000;
            cx.eval(t).abs() + cy.eval(t).abs() + cz.eval(t).abs()
        })
        .unwrap();

    // The inputs are constructed so that any collisions happen within the first
    // 40 ticks, unfortunately making any interesting solutions obsolete.
    let mut collisions = FastHashMap::with_capacity(particles.len());
    let mut removed = vec![-1; particles.len()];
    for t in 0..40 {
        collisions.clear();
        for (i, &(cx, cy, cz)) in particles.iter().enumerate() {
            if removed[i] == -1 {
                let pos = (cx.eval(t), cy.eval(t), cz.eval(t));
                if let Some(j) = collisions.insert(pos, i) {
                    removed[i] = t;
                    removed[j] = t;
                }
            }
        }
    }
    let part2 = removed.iter().filter(|&&r| r == -1).count();

    Ok((part1, part2))
}

#[allow(dead_code, reason = "alternative implementation")]
fn part2_proper(particles: &[(Coordinate, Coordinate, Coordinate)]) -> usize {
    // The position of a particle at time t is given by:
    //
    // f(t) = p + v * t + (a * t * (t + 1)) / 2
    //      = a / 2 * t^2 + (v + a / 2) * t + p
    //
    // To find the time where two particles collide, we need to solve for f_1(t)
    // = f_2(t), which is equivalent to f_1(t) - f_2(t) = 0. We can apply the
    // quadratic formula to the x component to find the two roots, discard any
    // are negative or not integers, and finally check them against the y and z
    // components.
    let indices: Vec<_> = (0..particles.len()).tuple_combinations().collect();
    let mut collisions: Vec<_> = indices
        .into_par_iter()
        .filter_map(|(i, j)| {
            let (c1x, c1y, c1z) = particles[i];
            let (c2x, c2y, c2z) = particles[j];
            let t = c1x
                .roots(c2x)
                .filter(|&t| c1y.eval(t) == c2y.eval(t) && c1z.eval(t) == c2z.eval(t))
                .min()?;
            Some((t, i, j))
        })
        .collect();
    collisions.sort_unstable_by_key(|&(t, _, _)| t);

    let mut removed = vec![None; particles.len()];
    for (t, i, j) in collisions {
        let occurs = (removed[i].is_none() || removed[i] == Some(t))
            && (removed[j].is_none() || removed[j] == Some(t));
        if occurs {
            removed[i] = Some(t);
            removed[j] = Some(t);
        }
    }
    removed.into_iter().filter(Option::is_none).count()
}

#[derive(Clone, Copy)]
struct Coordinate(i64, i64, i64);

impl Coordinate {
    fn eval(self, t: i64) -> i64 {
        self.0 + self.1 * t + self.2 * t * (t + 1) / 2
    }

    fn roots(self, other: Coordinate) -> impl Iterator<Item = i64> {
        // All coefficients multiplied by 2 to avoid fractions
        let a = self.2 - other.2;
        let b = 2 * (self.1 - other.1) + a;
        let c = 2 * (self.0 - other.0);
        let root_term = (b * b - 4 * a * c).checked_isqrt();
        [1, -1].into_iter().filter_map(move |f| {
            let t = (-b + f * root_term?).checked_div(2 * a)?;
            (t >= 0).then_some(t)
        })
    }
}
