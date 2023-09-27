use std::ops::{Index, IndexMut};

use state::{State, StateId};

use crate::alphabet::Alphabet;
use crate::util::arena::Arena;

pub mod state;

#[cfg(feature = "serde")]
mod serde;

/// Moore machine is a tuple $(Q, q_0, \Sigma, \Lambda, \delta, \omega)$, where
/// - $Q$ is a set of states,
/// - $q_0$ is an initial (start) state,
/// - $\Sigma$ is an input alphabet,
/// - $\Lambda$ is an output alphabet,
/// - $\delta : Q \times \Sigma \to Q$ is a transition function,
/// - $\omega : Q \to \Lambda$ is an output function.
#[derive(Debug)]
pub struct Moore<I: Alphabet, O: Alphabet> {
    states: Arena<State<I, O>>,
}

impl<I: Alphabet, O: Alphabet> Moore<I, O> {
    pub fn new() -> Self {
        Self {
            states: Arena::new(),
        }
    }

    pub fn add_state(&mut self, output: O) -> StateId {
        self.states.alloc_with_id(|id| State::new(id, output))
    }

    pub fn add_transition(&mut self, from: StateId, input: I, to: StateId) {
        self.state_mut(from).add_transition(input, to);
    }

    pub fn state(&self, index: StateId) -> &State<I, O> {
        &self.states[index]
    }
    pub fn state_mut(&mut self, index: StateId) -> &mut State<I, O> {
        &mut self.states[index]
    }

    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    pub fn num_transitions(&self) -> usize {
        self.states().map(|state| state.num_transitions()).sum()
    }

    pub fn states(&self) -> impl Iterator<Item = &State<I, O>> {
        self.states.iter()
    }
    pub fn states_mut(&mut self) -> impl Iterator<Item = &mut State<I, O>> {
        self.states.iter_mut()
    }

    pub fn transitions(&self) -> impl Iterator<Item = (&State<I, O>, I, &State<I, O>)> + '_ {
        self.states().flat_map(move |state| {
            state
                .transitions()
                .map(move |(input, to)| (state, input, self.state(to)))
        })
    }
}

impl<I: Alphabet, O: Alphabet> Default for Moore<I, O> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Alphabet, O: Alphabet> Index<StateId> for Moore<I, O> {
    type Output = State<I, O>;

    fn index(&self, index: StateId) -> &Self::Output {
        self.state(index)
    }
}

impl<I: Alphabet, O: Alphabet> IndexMut<StateId> for Moore<I, O> {
    fn index_mut(&mut self, index: StateId) -> &mut Self::Output {
        self.state_mut(index)
    }
}

impl<I: Alphabet, O: Alphabet> Moore<I, O> {
    pub fn next(&self, current_state: StateId, input: I) -> Option<(StateId, O)> {
        self.state(current_state)
            .next(input)
            .map(|next_state| (next_state, self.state(next_state).output))
    }

    pub fn run<'a, Inputs>(&'a self, inputs: Inputs) -> impl Iterator<Item = (I, StateId, O)> + '_
    where
        Inputs: IntoIterator<Item = I>,
        <Inputs as IntoIterator>::IntoIter: 'a,
    {
        let mut current_state = 0;
        inputs.into_iter().map(move |input: I| {
            let (next_state, output) = self.next(current_state, input).unwrap();
            current_state = next_state;
            (input, current_state, output)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moore_run() {
        let mut fsm = Moore::new();
        let s0 = fsm.add_state(0);
        let s1 = fsm.add_state(1);
        let s2 = fsm.add_state(2);
        fsm.add_transition(s0, 0, s0);
        fsm.add_transition(s0, 1, s1);
        fsm.add_transition(s1, 0, s2);
        fsm.add_transition(s1, 1, s0);
        fsm.add_transition(s2, 0, s1);
        fsm.add_transition(s2, 1, s2);

        let inputs = vec![1, 0, 1, 0, 1, 1, 0];
        println!("Running Moore machine on {:?}", inputs);
        let mut current_state = 0;
        let mut outputs = vec![fsm.state(current_state).output];
        println!("initial state {}, output {}", current_state, outputs[0]);
        for (input, new_state, output) in fsm.run(inputs) {
            println!(
                "state {}, input {}, new_state {}, output {}",
                current_state, input, new_state, output
            );
            outputs.push(output);
            current_state = new_state;
        }
        assert_eq!(outputs, vec![0, 1, 2, 2, 1, 0, 1, 2]);
    }
}
