use std::collections::HashMap;

use crate::alphabet::Alphabet;

pub type StateId = usize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State<I: Alphabet, O: Alphabet> {
    pub id: StateId,
    transitions: HashMap<I, (StateId, O)>,
}

impl<I: Alphabet, O: Alphabet> State<I, O> {
    pub fn new(id: StateId) -> Self {
        Self {
            id,
            transitions: HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, input: I, to: StateId, output: O) {
        self.transitions.insert(input, (to, output));
    }

    pub fn num_transitions(&self) -> usize {
        self.transitions.len()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (I, StateId, O)> + '_ {
        self.transitions
            .iter()
            .map(|(&symbol, &(to, output))| (symbol, to, output))
    }

    pub fn next(&self, input: I) -> Option<(StateId, O)> {
        self.transitions.get(&input).copied()
    }
}
