use std::hash::Hash;

use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

pub type FastHashSet<T> = FxHashSet<T>;
pub type FastHashMap<K, V> = FxHashMap<K, V>;

pub trait FastHashCollectionExt {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}

#[allow(clippy::implicit_hasher)]
impl<T> FastHashCollectionExt for FastHashSet<T> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher)
    }
}

#[allow(clippy::implicit_hasher)]
impl<K, V> FastHashCollectionExt for FastHashMap<K, V> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher)
    }
}

#[derive(Debug, Clone)]
pub struct Indexer<T>(FastHashMap<T, usize>);

impl<T: Eq + Hash> Indexer<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn index_of(&mut self, value: T) -> usize {
        let len = self.0.len();
        *self.0.entry(value).or_insert(len)
    }

    #[must_use]
    pub fn as_map(&self) -> &FastHashMap<T, usize> {
        &self.0
    }
}

impl<T> Default for Indexer<T> {
    fn default() -> Self {
        Self(FastHashMap::default())
    }
}
