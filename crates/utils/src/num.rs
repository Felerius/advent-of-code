use std::iter;

use num::{PrimInt, one};

pub fn bits(mut m: impl PrimInt) -> impl Iterator<Item = usize> {
    iter::from_fn(move || {
        (!m.is_zero()).then(|| {
            let i = m.trailing_zeros() as usize;
            m = m & (m - one());
            i
        })
    })
}
