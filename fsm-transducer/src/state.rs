use crate::output::OutputEvent;
use crate::symbol::Symbol;
use crate::transition::Transition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub id: usize,
    pub event: OutputEvent,
    pub transitions: Vec<Transition>,
}

impl State {
    pub fn new(id: usize, event: OutputEvent) -> Self {
        State {
            id,
            event,
            transitions: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: Transition) {
        assert_eq!(transition.source, self.id);
        self.transitions.push(transition);
    }

    pub fn next(&self, input: Symbol) -> Option<(usize, usize)> {
        // Search for the first transition that matches the input:
        for (k, transition) in self.transitions.iter().enumerate() {
            if let Some(s) = transition.next(input) {
                return Some((k, s));
            }
        }
        None
    }
}
