use std::collections::HashMap;

use crate::alphabet::Alphabet;

pub type StateId = usize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State<I: Alphabet, O: Alphabet> {
    pub id: StateId,
    pub output: O,
    transitions: HashMap<I, StateId>,
}

impl<I: Alphabet, O: Alphabet> State<I, O> {
    pub fn new(id: StateId, output: O) -> Self {
        Self {
            id,
            output,
            transitions: HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, input: I, to: StateId) {
        self.transitions.insert(input, to);
    }

    pub fn num_transitions(&self) -> usize {
        self.transitions.len()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (I, StateId)> + '_ {
        self.transitions.iter().map(|(&symbol, &to)| (symbol, to))
    }

    pub fn next(&self, input: I) -> Option<StateId> {
        self.transitions.get(&input).copied()
    }
}
