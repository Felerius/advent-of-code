use std::{iter::FusedIterator, marker::PhantomData};

use anyhow::{anyhow, Result};
use arrayvec::ArrayVec;
use num::{Integer, Signed, Unsigned};

pub trait Input {
    fn unsigned_integers<T>(&self) -> IntegersUnsigned<'_, T>;
    fn signed_integers<T>(&self) -> IntegersSigned<'_, T>;

    fn unsigned_integers_n<T, const N: usize>(&self) -> Result<[T; N]>
    where
        T: Integer + Unsigned + From<u8>,
    {
        try_collect_n(self.unsigned_integers())
    }

    fn signed_integers_n<T, const N: usize>(&self) -> Result<[T; N]>
    where
        T: Integer + Signed + From<u8>,
    {
        try_collect_n(self.signed_integers())
    }
}

impl<T: AsRef<[u8]> + ?Sized> Input for T {
    fn unsigned_integers<I>(&self) -> IntegersUnsigned<'_, I> {
        IntegersUnsigned(self.as_ref(), PhantomData)
    }

    fn signed_integers<I>(&self) -> IntegersSigned<'_, I> {
        IntegersSigned(self.as_ref(), PhantomData)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntegersUnsigned<'a, T>(&'a [u8], PhantomData<T>);

impl<T: Integer + Unsigned + From<u8>> Iterator for IntegersUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (num, rem) = next_unsigned::<T>(self.0)?;
        self.0 = rem;
        Some(num)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (0, Some(len.div_ceil(2)))
    }
}

impl<T: Integer + Unsigned + From<u8>> FusedIterator for IntegersUnsigned<'_, T> {}

#[derive(Debug, Clone, Copy)]
pub struct IntegersSigned<'a, T>(&'a [u8], PhantomData<T>);

impl<T: Integer + Signed + From<u8>> Iterator for IntegersSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (num, rem) = next_signed::<T>(self.0)?;
        self.0 = rem;
        Some(num)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (0, Some(len.div_ceil(2)))
    }
}

impl<T: Integer + Signed + From<u8>> FusedIterator for IntegersSigned<'_, T> {}

fn next_unsigned<T: Integer + From<u8>>(s: &[u8]) -> Option<(T, &[u8])> {
    let start = s.iter().position(u8::is_ascii_digit)?;
    let mut num = T::zero();
    let mut s = &s[start..];
    while let Some((&c, tail)) = s.split_first().filter(|(c, _)| c.is_ascii_digit()) {
        num = num * T::from(10_u8) + T::from(c - b'0');
        s = tail;
    }

    Some((num, s))
}

fn next_signed<T: Integer + Signed + From<u8>>(s: &[u8]) -> Option<(T, &[u8])> {
    let start = s.iter().position(u8::is_ascii_digit)?;
    let (mut num, rem) = next_unsigned::<T>(&s[start..])?;
    if s[..start].ends_with(b"-") {
        num = -num;
    }

    Some((num, rem))
}

fn try_collect_n<T, const N: usize>(iter: impl Iterator<Item = T>) -> Result<[T; N]> {
    let mut arr = ArrayVec::<T, N>::new();
    for num in iter {
        arr.try_push(num)
            .map_err(|_| anyhow!("Expected only {N} integers, got at least one more"))?;
    }

    arr.into_inner()
        .map_err(|arr| anyhow!("Expected {} integers, got {}", N, arr.len()))
}
