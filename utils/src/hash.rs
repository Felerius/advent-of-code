use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

pub type FastHashSet<T> = FxHashSet<T>;
pub type FastHashMap<K, V> = FxHashMap<K, V>;

pub trait FastHashCollectionExt {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}

impl<T> FastHashCollectionExt for FastHashSet<T> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher::default())
    }
}

impl<K, V> FastHashCollectionExt for FastHashMap<K, V> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher::default())
    }
}
