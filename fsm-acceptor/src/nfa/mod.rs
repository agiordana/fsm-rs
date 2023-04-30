use std::collections::HashSet;
use std::ops::{Index, IndexMut};

use state::{State, StateId};

use crate::alphabet::Alphabet;
use crate::arena::Arena;

pub mod graphviz;
pub mod state;

#[derive(Debug)]
pub struct Nfa<A: Alphabet> {
    states: Arena<State<A>>,
}

impl<A: Alphabet> Nfa<A> {
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
        self.state_mut(from).transitions.insert(symbol, to);
    }

    pub fn add_epsilon_transition(&mut self, from: StateId, to: StateId) {
        self.state_mut(from).epsilon_transitions.insert(to);
    }

    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    pub fn num_transitions(&self) -> usize {
        self.states
            .iter()
            .map(|state| {
                state
                    .transitions
                    .iter_all()
                    .map(|(_, to)| to.len())
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn num_epsilon_transitions(&self) -> usize {
        self.states
            .iter()
            .map(|state| state.epsilon_transitions.len())
            .sum()
    }

    pub fn states(&self) -> impl Iterator<Item = &State<A>> {
        self.states.iter()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (&State<A>, A, &State<A>)> + '_ {
        self.states().flat_map(move |state| {
            state
                .transitions
                .iter_all()
                .flat_map(move |(symbol, to_all)| {
                    to_all
                        .iter()
                        .map(move |to| (state, *symbol, self.state(*to)))
                })
        })
    }

    pub fn epsilon_transitions(&self) -> impl Iterator<Item = (&State<A>, &State<A>)> + '_ {
        self.states().flat_map(move |state| {
            state
                .epsilon_transitions
                .iter()
                .map(move |to| (state, self.state(*to)))
        })
    }
}

impl<A: Alphabet> Default for Nfa<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Alphabet> Index<StateId> for Nfa<A> {
    type Output = State<A>;

    fn index(&self, index: StateId) -> &Self::Output {
        self.state(index)
    }
}

impl<A: Alphabet> IndexMut<StateId> for Nfa<A> {
    fn index_mut(&mut self, index: StateId) -> &mut Self::Output {
        self.state_mut(index)
    }
}

impl<A: Alphabet> Nfa<A> {
    pub fn next(&self, current_state: StateId, symbol: A) -> Option<Vec<StateId>> {
        self.state(current_state).next(symbol)
    }

    fn reach_epsilon(&self, state: StateId, visited: &mut HashSet<StateId>) {
        if visited.contains(&state) {
            return;
        }
        visited.insert(state);
        for &x in self.state(state).epsilon_transitions.iter() {
            self.reach_epsilon(x, visited);
        }
    }

    pub fn accepts<I>(&self, word: I) -> bool
    where
        I: IntoIterator<Item = A>,
    {
        if self.states.is_empty() {
            return false;
        }

        let mut current_states = HashSet::new();
        self.reach_epsilon(0, &mut current_states);

        for symbol in word {
            let mut next_states = HashSet::new();

            for state in current_states {
                if let Some(transitions) = self.next(state, symbol) {
                    for next_state in transitions {
                        next_states.insert(next_state);
                    }
                }
            }

            current_states = next_states;
            let mut visited = HashSet::new();
            for state in current_states {
                self.reach_epsilon(state, &mut visited);
            }
            current_states = visited;
        }

        current_states
            .into_iter()
            .any(|state| self.state(state).accepting)
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

        let mut nfa = Nfa::new();
        nfa.add_state(true);
        nfa.add_state(false);
        // Loops:
        nfa[0].transitions.insert(One, 0);
        nfa[1].transitions.insert(One, 1);
        // Transitions:
        nfa[0].transitions.insert(Zero, 1);
        nfa[1].transitions.insert(Zero, 0);

        // This NFA accepts all words with even number of Zeros
        assert!(nfa.accepts(vec![]));
        assert!(nfa.accepts(vec![One]));
        assert!(nfa.accepts(vec![One, One]));
        assert!(nfa.accepts(vec![Zero, Zero]));
        assert!(nfa.accepts(vec![Zero, One, One, Zero]));
        assert!(nfa.accepts(vec![Zero, One, Zero, One]));
        assert!(nfa.accepts(vec![Zero, Zero, One, One]));
        assert!(!nfa.accepts(vec![Zero]));
        assert!(!nfa.accepts(vec![Zero, One]));
        assert!(!nfa.accepts(vec![One, Zero]));
        assert!(!nfa.accepts(vec![One, One, Zero]));
        assert!(!nfa.accepts(vec![One, One, Zero, Zero, Zero]));
        assert!(!nfa.accepts(vec![One, One, Zero, Zero, One, Zero]));
    }
}
