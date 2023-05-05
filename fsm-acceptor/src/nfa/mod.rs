use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::{Index, IndexMut};

use state::{State, StateId};

use crate::alphabet::Alphabet;
use crate::dfa::Dfa;
use crate::util::arena::Arena;
use crate::util::dfs::multi_dfs;

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

    pub fn add_state(&mut self, accepting: bool) -> StateId {
        self.states.alloc_with_id(|id| State::new(id, accepting))
    }

    pub fn add_transition(&mut self, from: StateId, symbol: A, to: StateId) {
        self.state_mut(from).add_transition(symbol, to);
    }

    pub fn add_epsilon_transition(&mut self, from: StateId, to: StateId) {
        self.state_mut(from).add_epsilon_transition(to);
    }

    pub fn state(&self, index: StateId) -> &State<A> {
        &self.states[index]
    }
    pub fn state_mut(&mut self, index: StateId) -> &mut State<A> {
        &mut self.states[index]
    }

    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    pub fn num_transitions(&self) -> usize {
        self.states().map(|state| state.num_transitions()).sum()
    }

    pub fn num_epsilon_transitions(&self) -> usize {
        self.states().map(|state| state.next_epsilon().len()).sum()
    }

    pub fn accepting(&self, state: StateId) -> bool {
        self.state(state).accepting
    }

    pub fn states(&self) -> impl Iterator<Item = &State<A>> {
        self.states.iter()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (&State<A>, A, &State<A>)> + '_ {
        self.states().flat_map(move |state| {
            state
                .transitions()
                .map(move |(symbol, to)| (state, symbol, self.state(to)))
        })
    }

    pub fn epsilon_transitions(&self) -> impl Iterator<Item = (&State<A>, &State<A>)> + '_ {
        self.states().flat_map(move |state| {
            state
                .next_epsilon()
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

    fn epsilon_closure(&self, start: StateId) -> impl Iterator<Item = StateId> + '_ {
        self.multi_epsilon_closure(vec![start])
    }

    fn multi_epsilon_closure(&self, start: Vec<StateId>) -> impl Iterator<Item = StateId> + '_ {
        multi_dfs(start, |state| self.next_epsilon(state).iter().copied())
    }

    fn any_accepting(&self, states: impl IntoIterator<Item = StateId>) -> bool {
        states.into_iter().any(|s| self.accepting(s))
    }

    pub fn accepts(&self, word: impl IntoIterator<Item = A>) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut current = self.epsilon_closure(0).collect::<BTreeSet<_>>();

        for symbol in word {
            let mut next = BTreeSet::new();
            for state in current {
                if let Some(next_states) = self.next(state, symbol) {
                    next.extend(self.multi_epsilon_closure(next_states.clone()))
                }
            }
            current = next;
        }

        self.any_accepting(current)
    }

    pub fn to_dfa(&self, alphabet: &[A]) -> Dfa<A> {
        let mut dfa = Dfa::new();
        let mut state_map = HashMap::new();
        let mut queue = Vec::new();

        let initial_nfa_state = self.epsilon_closure(0).collect::<BTreeSet<_>>();
        let initial_accepting = self.any_accepting(initial_nfa_state.iter().copied());
        let initial_dfa_state = dfa.add_state(initial_accepting);
        state_map.insert(initial_nfa_state.clone(), initial_dfa_state);
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
                            let accepting = self.any_accepting(next_nfa_state.iter().copied());
                            let new_dfa_state = dfa.add_state(accepting);
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
        let a = nfa.add_state(true);
        let b = nfa.add_state(false);
        // Loops:
        nfa.add_transition(a, One, a);
        nfa.add_transition(b, One, b);
        // Transitions:
        nfa.add_transition(a, Zero, b);
        nfa.add_transition(b, Zero, a);

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
        let a = nfa.add_state(false);
        let b = nfa.add_state(true);
        nfa.add_epsilon_transition(a, a);
        nfa.add_transition(a, Zero, a);
        nfa.add_transition(a, One, a);
        nfa.add_transition(a, One, b);
        nfa.add_transition(b, Zero, a);
        nfa.add_transition(b, One, b);

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
        let a = nfa.add_state(false);
        let b = nfa.add_state(true);
        nfa.add_epsilon_transition(a, a);
        nfa.add_transition(a, Zero, a);
        nfa.add_transition(a, One, a);
        nfa.add_transition(a, One, b);
        nfa.add_transition(b, Zero, a);
        nfa.add_transition(b, One, b);

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
