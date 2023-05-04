use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::{Index, IndexMut};

use state::{State, StateId};

use crate::alphabet::Alphabet;
use crate::dfa::Dfa;
use crate::util::arena::Arena;

pub mod graphviz;
pub mod state;

#[cfg(feature = "serde")]
mod serde;

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
        self.state_mut(from).add_transition(symbol, to);
    }

    pub fn add_epsilon_transition(&mut self, from: StateId, to: StateId) {
        self.state_mut(from).add_epsilon_transition(to);
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
    pub fn next(&self, state: StateId, symbol: A) -> Option<&Vec<StateId>> {
        self.state(state).next(symbol)
    }

    pub fn next_epsilon(&self, state: StateId) -> &HashSet<StateId> {
        self.state(state).next_epsilon()
    }

    fn epsilon_closure(&self, start: StateId) -> BTreeSet<StateId> {
        self.multi_epsilon_closure(vec![start])
    }

    fn multi_epsilon_closure(&self, start: Vec<StateId>) -> BTreeSet<StateId> {
        let mut visited = BTreeSet::new();
        let mut stack = start;

        while let Some(state) = stack.pop() {
            if visited.insert(state) {
                for &next_state in self.next_epsilon(state) {
                    stack.push(next_state);
                }
            }
        }

        visited
    }

    fn multi_next_epsilon_closure(
        &self,
        current_states: impl IntoIterator<Item = StateId>,
        symbol: A,
    ) -> BTreeSet<StateId> {
        let mut res = BTreeSet::new();
        for state in current_states {
            if let Some(next_states) = self.next(state, symbol) {
                res.extend(self.multi_epsilon_closure(next_states.clone()))
            }
        }
        res
    }

    fn is_accepting(&self, states: impl IntoIterator<Item = StateId>) -> bool {
        states.into_iter().any(|s| self.state(s).accepting)
    }

    pub fn accepts<I>(&self, word: I) -> bool
    where
        I: IntoIterator<Item = A>,
    {
        if self.states.is_empty() {
            return false;
        }

        let mut current_states = self.epsilon_closure(0);

        for symbol in word {
            current_states = self.multi_next_epsilon_closure(current_states, symbol);
        }

        self.is_accepting(current_states)
    }

    pub fn to_dfa(&self, alphabet: &[A]) -> Dfa<A> {
        let mut dfa = Dfa::new();
        let mut state_map = HashMap::new();
        let mut queue = Vec::new();

        let initial_nfa_state = self.epsilon_closure(0);
        let initial_state = dfa.add_state(self.is_accepting(initial_nfa_state.iter().copied()));
        state_map.insert(initial_nfa_state.clone(), initial_state);
        queue.push(initial_nfa_state);

        while let Some(current_nfa_state) = queue.pop() {
            let current_state = state_map[&current_nfa_state];

            for &symbol in alphabet {
                let mut next_nfa_state = BTreeSet::new();
                for &nfa_state in &current_nfa_state {
                    if let Some(next) = self.next(nfa_state, symbol) {
                        next_nfa_state.extend(self.multi_epsilon_closure(next.clone()));
                    }
                }

                if !next_nfa_state.is_empty() {
                    let next_dfa_state =
                        *state_map.entry(next_nfa_state.clone()).or_insert_with(|| {
                            let new_dfa_state =
                                dfa.add_state(self.is_accepting(next_nfa_state.iter().copied()));
                            queue.push(next_nfa_state);
                            new_dfa_state
                        });
                    dfa.add_transition(current_state, symbol, next_dfa_state);
                }
            }
        }

        dfa
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
        nfa.add_transition(0, One, 0);
        nfa.add_transition(1, One, 1);
        // Transitions:
        nfa.add_transition(0, Zero, 1);
        nfa.add_transition(1, Zero, 0);

        // This NFA (actually, DFA) accepts all words with even number of Zeros
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

    #[test]
    fn test_simple_nfa() {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
        enum Sigma {
            Zero,
            One,
        }
        use Sigma::*;

        let mut nfa = Nfa::new();
        nfa.add_state(false);
        nfa.add_state(true);
        nfa.add_epsilon_transition(0, 0);
        nfa.add_transition(0, Zero, 0);
        nfa.add_transition(0, One, 0);
        nfa.add_transition(0, One, 1);
        nfa.add_transition(1, Zero, 0);
        nfa.add_transition(1, One, 1);

        // This NFA accepts only words ending with One
        assert!(nfa.accepts(vec![One]));
        assert!(nfa.accepts(vec![Zero, One]));
        assert!(nfa.accepts(vec![Zero, Zero, One]));
        assert!(nfa.accepts(vec![Zero, One, Zero, One]));
        assert!(nfa.accepts(vec![One, Zero, Zero, Zero, One]));
        assert!(!nfa.accepts(vec![One, Zero]));
        assert!(!nfa.accepts(vec![One, Zero, Zero]));
        assert!(!nfa.accepts(vec![One, Zero, Zero, Zero]));
        assert!(!nfa.accepts(vec![One, Zero, Zero, Zero]));
        assert!(!nfa.accepts(vec![One, One, Zero, Zero]));
    }

    #[test]
    fn test_nfa_to_dfa() {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
        enum Sigma {
            Zero,
            One,
        }
        use Sigma::*;

        let mut nfa = Nfa::new();
        nfa.add_state(false);
        nfa.add_state(true);
        nfa.add_epsilon_transition(0, 0);
        nfa.add_transition(0, Zero, 0);
        nfa.add_transition(0, One, 0);
        nfa.add_transition(0, One, 1);
        nfa.add_transition(1, Zero, 0);
        nfa.add_transition(1, One, 1);

        let dfa = nfa.to_dfa(&[Zero, One]);

        // This DFA accepts only words ending with One
        assert!(dfa.accepts(vec![One]));
        assert!(dfa.accepts(vec![Zero, One]));
        assert!(dfa.accepts(vec![Zero, Zero, One]));
        assert!(dfa.accepts(vec![Zero, One, Zero, One]));
        assert!(dfa.accepts(vec![One, Zero, Zero, Zero, One]));
        assert!(!dfa.accepts(vec![One, Zero]));
        assert!(!dfa.accepts(vec![One, Zero, Zero]));
        assert!(!dfa.accepts(vec![One, Zero, Zero, Zero]));
        assert!(!dfa.accepts(vec![One, Zero, Zero, Zero]));
        assert!(!dfa.accepts(vec![One, One, Zero, Zero]));
    }
}
