use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

pub trait Set<T> {
    fn new() -> Self;
    fn insert(&mut self, item: T) -> bool;
    fn contains(&self, item: &T) -> bool;
}

impl<T> Set<T> for HashSet<T>
where
    T: Hash + Eq,
{
    fn new() -> Self {
        HashSet::new()
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Set<T> for BTreeSet<T>
where
    T: Ord,
{
    fn new() -> Self {
        BTreeSet::new()
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }
}
