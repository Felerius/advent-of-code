use std::fmt::{self, Debug, Display, Formatter};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PuzzleId {
    pub year: u16,
    pub day: u8,
}

impl Display for PuzzleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.day)
    }
}

pub struct Output {
    pub part1: Box<dyn Display>,
    pub part2: Box<dyn Display>,
}

impl Debug for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Output")
            .field("part1", &DebugWithDisplay(&self.part1))
            .field("part2", &DebugWithDisplay(&self.part2))
            .finish()
    }
}

impl<S: Display + 'static, T: Display + 'static> From<(S, T)> for Output {
    fn from((part1, part2): (S, T)) -> Self {
        Self {
            part1: Box::new(part1),
            part2: Box::new(part2),
        }
    }
}

pub type Solution = fn(&str) -> Result<Output>;

#[macro_export]
macro_rules! collect {
    ($year:literal; $($days:literal),* $(,)?) => {
        $(
            $crate::__macro_support::concat_idents!(mod_name = day, $days {
                mod mod_name;
            });
        )*

        pub const SOLUTIONS: &[($crate::PuzzleId, $crate::Solution)] = &[
            $(
                $crate::__macro_support::concat_idents!(mod_name = day, $days {
                    {
                        fn wrapper(input: &str) -> $crate::__macro_support::Result<$crate::Output> {
                            let output = mod_name::run(input);
                            $crate::__macro_support::IntoResultOutput::into(output)
                        }

                        let id = $crate::PuzzleId {
                            year: $year,
                            #[allow(clippy::zero_prefixed_literal)]
                            day: $days,
                        };
                        (id, wrapper)
                    }
                }),
            )*
        ];
    };
}

#[doc(hidden)]
pub mod __macro_support {
    pub use anyhow::Result;
    pub use concat_idents::concat_idents;

    use super::Output;

    pub trait IntoResultOutput {
        fn into(self) -> Result<Output>;
    }

    impl<T: Into<Output>> IntoResultOutput for T {
        fn into(self) -> Result<Output> {
            anyhow::Ok(self.into())
        }
    }

    impl<T, E> IntoResultOutput for Result<T, E>
    where
        T: Into<Output>,
        anyhow::Error: From<E>,
    {
        fn into(self) -> Result<Output> {
            Ok(self?.into())
        }
    }
}

struct DebugWithDisplay<'a, T>(&'a T);

impl<T: Display> Debug for DebugWithDisplay<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
