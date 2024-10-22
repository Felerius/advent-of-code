use arrayvec::ArrayVec;
use num::{Integer, Signed};

pub fn integers<T, const N: usize>(input: impl AsRef<[u8]>) -> [T; N]
where
    T: Integer + From<u8>,
{
    let mut input = input.as_ref();
    let mut nums = ArrayVec::<T, N>::new();
    loop {
        let Some(off) = input.iter().position(|&c| c.is_ascii_digit()) else {
            break;
        };

        input = &input[off..];
        let mut num = T::zero();
        while let Some((&c, tail)) = input.split_first().filter(|(c, _)| c.is_ascii_digit()) {
            num = num * T::from(10_u8) + T::from(c - b'0');
            input = tail;
        }
        nums.try_push(num)
            .unwrap_or_else(|_| panic!("Expected only {} integers", N));
    }

    nums.into_inner()
        .unwrap_or_else(|nums| panic!("Expected {} integers, got {}", N, nums.len()))
}

pub fn signed_integers<T, const N: usize>(input: impl AsRef<[u8]>) -> [T; N]
where
    T: Integer + From<u8> + Signed,
{
    let mut input = input.as_ref();
    let mut nums = ArrayVec::<T, N>::new();
    loop {
        let Some(off) = input.iter().position(|&c| c.is_ascii_digit() || c == b'-') else {
            break;
        };

        input = &input[off..];
        let negative = if input[0] == b'-' {
            input = &input[1..];
            if input.first().is_none_or(|&c| !c.is_ascii_digit()) {
                continue;
            }
            true
        } else {
            false
        };

        let mut num = T::zero();
        while let Some((&c, tail)) = input.split_first().filter(|(c, _)| c.is_ascii_digit()) {
            num = num * T::from(10_u8) + T::from(c - b'0');
            input = tail;
        }

        num = if negative { -num } else { num };
        nums.try_push(num)
            .unwrap_or_else(|_| panic!("Expected only {} integers", N));
    }

    nums.into_inner()
        .unwrap_or_else(|nums| panic!("Expected {} integers, got {}", N, nums.len()))
}
