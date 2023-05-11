use std::fmt::Debug;
use std::hash::Hash;

pub trait Alphabet: Hash + Eq + Debug + Clone + Copy + Ord {}

impl<T> Alphabet for T where T: Hash + Eq + Debug + Clone + Copy + Ord {}
