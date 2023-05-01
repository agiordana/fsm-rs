use std::collections::HashSet;

use multimap::MultiMap;

use crate::alphabet::Alphabet;

pub type StateId = usize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct State<A: Alphabet> {
    pub id: StateId,
    pub accepting: bool,
    pub transitions: MultiMap<A, StateId>,
    pub epsilon_transitions: HashSet<StateId>,
}

impl<A: Alphabet> State<A> {
    pub fn new(id: StateId, accepting: bool) -> Self {
        Self {
            id,
            accepting,
            transitions: MultiMap::new(),
            epsilon_transitions: HashSet::new(),
        }
    }

    pub fn next(&self, symbol: A) -> Option<Vec<StateId>> {
        self.transitions.get_vec(&symbol).cloned()
    }
}
