use std::ops::{Index, IndexMut};

use state::{State, StateId};

use crate::alphabet::Alphabet;
use crate::arena::Arena;

pub mod graphviz;
pub mod state;

#[derive(Debug)]
pub struct Dfa<A: Alphabet> {
    states: Arena<State<A>>,
}

impl<A: Alphabet> Dfa<A> {
    pub fn new() -> Self {
        Self {
            states: Arena::new(),
        }
    }

    pub fn state(&self, index: StateId) -> &State<A> {
        &self.states[index]
    }
    pub fn state_mut(&mut self, index: StateId) -> &mut State<A> {
        &mut self.states[index]
    }

    pub fn add_state(&mut self, accepting: bool) -> StateId {
        self.states.alloc_with_id(|id| State::new(id, accepting))
    }

    pub fn add_transition(&mut self, from: StateId, symbol: A, to: StateId) {
        self.state_mut(from).add_transition(symbol, to);
    }

    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    pub fn num_transitions(&self) -> usize {
        self.states
            .iter()
            .map(|state| state.transitions.len())
            .sum()
    }

    pub fn states(&self) -> impl Iterator<Item = &State<A>> {
        self.states.iter()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (&State<A>, A, &State<A>)> + '_ {
        self.states().flat_map(move |state| {
            state
                .transitions
                .iter()
                .map(move |(symbol, to)| (state, *symbol, self.state(*to)))
        })
    }
}

impl<A: Alphabet> Default for Dfa<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Alphabet> Index<StateId> for Dfa<A> {
    type Output = State<A>;

    fn index(&self, index: StateId) -> &Self::Output {
        self.state(index)
    }
}

impl<A: Alphabet> IndexMut<StateId> for Dfa<A> {
    fn index_mut(&mut self, index: StateId) -> &mut Self::Output {
        self.state_mut(index)
    }
}

impl<A: Alphabet> Dfa<A> {
    pub fn next(&self, current_state: StateId, symbol: A) -> Option<StateId> {
        self.state(current_state).next(symbol)
    }

    pub fn accepts<I>(&self, word: I) -> bool
    where
        I: IntoIterator<Item = A>,
    {
        if self.states.is_empty() {
            return false;
        }
        let mut current_state = 0;
        for symbol in word.into_iter() {
            if let Some(next_state) = self.next(current_state, symbol) {
                current_state = next_state;
            } else {
                return false;
            }
        }
        self.state(current_state).accepting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dfa() {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
        enum Sigma {
            Zero,
            One,
        }
        use Sigma::*;

        let mut dfa = Dfa::new();
        dfa.add_state(true);
        dfa.add_state(false);
        // Loops:
        dfa[0].transitions.insert(One, 0);
        dfa[1].transitions.insert(One, 1);
        // Transitions:
        dfa[0].transitions.insert(Zero, 1);
        dfa[1].transitions.insert(Zero, 0);

        // NFA accepts all words with even number of Zeros
        assert!(dfa.accepts(vec![]));
        assert!(dfa.accepts(vec![One]));
        assert!(dfa.accepts(vec![One, One]));
        assert!(dfa.accepts(vec![Zero, Zero]));
        assert!(dfa.accepts(vec![Zero, One, One, Zero]));
        assert!(dfa.accepts(vec![Zero, One, Zero, One]));
        assert!(dfa.accepts(vec![Zero, Zero, One, One]));
        assert!(!dfa.accepts(vec![Zero]));
        assert!(!dfa.accepts(vec![Zero, One]));
        assert!(!dfa.accepts(vec![One, Zero]));
        assert!(!dfa.accepts(vec![One, One, Zero]));
        assert!(!dfa.accepts(vec![One, One, Zero, Zero, Zero]));
        assert!(!dfa.accepts(vec![One, One, Zero, Zero, One, Zero]));
    }
}
