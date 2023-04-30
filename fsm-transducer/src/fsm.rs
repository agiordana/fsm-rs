use std::collections::HashMap;

use crate::state::State;
use crate::transition::Transition;

pub struct FSM {
    pub(crate) states: HashMap<usize, State>,
}

impl FSM {
    pub fn new() -> Self {
        FSM {
            states: HashMap::new(),
        }
    }

    pub fn state(&self, id: usize) -> Option<&State> {
        self.states.get(&id)
    }

    pub fn add_state(&mut self, state: State) {
        self.states.insert(state.id, state);
    }

    pub fn add_transition(&mut self, transition: Transition) {
        let state = self.states.get_mut(&transition.source).unwrap();
        state.add_transition(transition);
    }
}
