use anyhow::Result;
pub use register_macro::register;

pub type SolutionFunction = fn(&str) -> Result<(String, String)>;

#[derive(Debug, Clone, Copy)]
pub struct RegisteredFunction {
    pub module_path: &'static str,
    pub name: &'static str,
    pub file: &'static str,
    pub func: SolutionFunction,
}

impl RegisteredFunction {
    #[must_use]
    pub fn all() -> &'static [Self] {
        &__macro_support::REGISTERED_FUNCTIONS
    }
}

#[doc(hidden)]
pub mod __macro_support {
    pub use linkme;
    pub use linkme::distributed_slice;

    #[distributed_slice]
    pub static REGISTERED_FUNCTIONS: [super::RegisteredFunction];

    pub trait NormalizeOutput {
        fn normalize(self) -> anyhow::Result<(String, String)>;
    }

    impl<S: ToString, T: ToString> NormalizeOutput for (S, T) {
        fn normalize(self) -> anyhow::Result<(String, String)> {
            Ok((self.0.to_string(), self.1.to_string()))
        }
    }

    impl<T: NormalizeOutput, E> NormalizeOutput for Result<T, E>
    where
        anyhow::Error: From<E>,
    {
        fn normalize(self) -> anyhow::Result<(String, String)> {
            self?.normalize()
        }
    }
}
