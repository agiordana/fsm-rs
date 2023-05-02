use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

pub trait Set<T> {
    fn insert(&mut self, item: T) -> bool;
    fn contains(&self, item: &T) -> bool;
}

impl<T> Set<T> for HashSet<T>
where
    T: Hash + Eq,
{
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
    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }
}
