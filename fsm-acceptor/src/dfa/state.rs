use std::collections::HashMap;

use crate::alphabet::Alphabet;

pub type StateId = usize;

#[derive(Debug)]
pub struct State<A: Alphabet> {
    pub id: StateId,
    pub accepting: bool,
    pub transitions: HashMap<A, StateId>,
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

    pub fn next(&self, symbol: A) -> Option<StateId> {
        self.transitions.get(&symbol).copied()
    }
}
