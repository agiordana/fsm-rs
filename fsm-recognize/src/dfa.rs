use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::alphabet::Alphabet;

#[derive(Debug)]
pub struct State<A: Alphabet> {
    #[allow(dead_code)]
    id: usize,
    accepting: bool,
    transitions: HashMap<A, usize>,
}

impl<A: Alphabet> State<A> {
    pub fn new(id: usize, accepting: bool) -> Self {
        Self {
            id,
            accepting,
            transitions: HashMap::new(),
        }
    }

    pub fn next(&self, symbol: A) -> Option<usize> {
        self.transitions.get(&symbol).copied()
    }
}

pub struct Dfa<A: Alphabet> {
    states: Vec<State<A>>,
}

impl<A: Alphabet> Dfa<A> {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    pub fn new_state(&mut self, accepting: bool) -> usize {
        let id = self.states.len();
        self.states.push(State::new(id, accepting));
        id
    }

    pub fn state(&self, index: usize) -> &State<A> {
        &self.states[index]
    }
    pub fn state_mut(&mut self, index: usize) -> &mut State<A> {
        &mut self.states[index]
    }
}

impl<A: Alphabet> Index<usize> for Dfa<A> {
    type Output = State<A>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.states[index]
    }
}

impl<A: Alphabet> IndexMut<usize> for Dfa<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.states[index]
    }
}

impl<A: Alphabet> Dfa<A> {
    pub fn next(&self, current_state: usize, symbol: A) -> Option<usize> {
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
        dfa.new_state(true);
        dfa.new_state(false);
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
