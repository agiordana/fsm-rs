use std::collections::HashMap;

use crate::alphabet::Alphabet;

pub type StateId = usize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State<A: Alphabet> {
    pub id: StateId,
    pub accepting: bool,
    transitions: HashMap<A, StateId>,
}

impl<A: Alphabet> State<A> {
    pub fn new(id: StateId, accepting: bool) -> Self {
        Self {
            id,
            accepting,
            transitions: HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, symbol: A, to: StateId) {
        self.transitions.insert(symbol, to);
    }

    pub fn num_transitions(&self) -> usize {
        self.transitions.len()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (A, StateId)> + '_ {
        self.transitions.iter().map(|(&symbol, &to)| (symbol, to))
    }

    pub fn next(&self, symbol: A) -> Option<StateId> {
        self.transitions.get(&symbol).copied()
    }
}
