use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct State {
    id: usize,
    accepting: bool,
    transitions: HashMap<char, usize>,
    epsilon_transitions: HashSet<usize>,
}

impl State {
    pub fn new(id: usize, accepting: bool) -> Self {
        Self {
            id,
            accepting,
            transitions: HashMap::new(),
            epsilon_transitions: HashSet::new(),
        }
    }
}

#[derive(Debug)]
pub struct Fragment {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct States {
    states: Vec<State>,
}

impl States {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    /// Create a new state and return its index.
    pub fn new_state(&mut self, accepting: bool) -> usize {
        let id = self.states.len();
        self.states.push(State::new(id, accepting));
        id
    }

    pub fn state(&self, index: usize) -> &State {
        &self.states[index]
    }
    pub fn state_mut(&mut self, index: usize) -> &mut State {
        &mut self.states[index]
    }

    pub fn parse(&mut self, pattern: &str) -> Fragment {
        let postfix = to_postfix(&insert_explicit_concat_operator(pattern));
        dbg!(&pattern);
        dbg!(&postfix);
        let mut stack = Vec::new();
        for token in postfix.chars() {
            match token {
                '\0' => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();
                    stack.push(self.concat(f1, f2));
                }
                '|' => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();
                    stack.push(self.union(f1, f2));
                }
                '*' => {
                    let f = stack.pop().unwrap();
                    stack.push(self.closure(f));
                }
                c => {
                    stack.push(self.symbol(c));
                }
            }
        }
        stack
            .into_iter()
            .reduce(|f1, f2| self.concat(f1, f2))
            .unwrap()
    }

    pub fn symbol(&mut self, c: char) -> Fragment {
        let start = self.new_state(false);
        let end = self.new_state(true);
        // Connect the start state to the end state with the given symbol
        self.state_mut(start).transitions.insert(c, end);
        Fragment { start, end }
    }

    pub fn concat(&mut self, f1: Fragment, f2: Fragment) -> Fragment {
        // Connect the old end state to the new start state
        self.state_mut(f1.end).epsilon_transitions.insert(f2.start);
        self.state_mut(f1.end).accepting = false;
        Fragment {
            start: f1.start,
            end: f2.end,
        }
    }

    pub fn union(&mut self, f1: Fragment, f2: Fragment) -> Fragment {
        let start = self.new_state(false);
        let end = self.new_state(true);
        // Connect the new start state to the old start states
        self.state_mut(start).epsilon_transitions.insert(f1.start);
        self.state_mut(start).epsilon_transitions.insert(f2.start);
        // Connect the old end states to the new end state
        self.state_mut(f1.end).epsilon_transitions.insert(end);
        self.state_mut(f2.end).epsilon_transitions.insert(end);
        // Make sure the old end states are no longer accepting
        self.state_mut(f1.end).accepting = false;
        self.state_mut(f2.end).accepting = false;
        Fragment { start, end }
    }

    pub fn closure(&mut self, f: Fragment) -> Fragment {
        let start = self.new_state(false);
        let end = self.new_state(true);
        self.state_mut(start).epsilon_transitions.insert(f.start);
        self.state_mut(start).epsilon_transitions.insert(end);
        self.state_mut(f.end).epsilon_transitions.insert(f.start);
        self.state_mut(f.end).epsilon_transitions.insert(end);
        self.state_mut(f.end).accepting = false;
        Fragment { start, end }
    }

    fn reach_epsilon(&self, state: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&state) {
            return;
        }
        visited.insert(state);
        for &x in self.state(state).epsilon_transitions.iter() {
            self.reach_epsilon(x, visited);
        }
    }

    pub fn matches(&self, start: usize, s: &str) -> bool {
        let mut current_states = HashSet::new();
        self.reach_epsilon(start, &mut current_states);

        for c in s.chars() {
            let mut next_states = HashSet::new();

            for &state in current_states.iter() {
                if let Some(&next_state) = self.state(state).transitions.get(&c) {
                    self.reach_epsilon(next_state, &mut next_states);
                } else if let Some(&next_state) = self.state(state).transitions.get(&'.') {
                    self.reach_epsilon(next_state, &mut next_states);
                }
            }

            current_states = next_states;
        }

        current_states
            .iter()
            .any(|&state| self.state(state).accepting)
    }
}

impl Index<usize> for States {
    type Output = State;

    fn index(&self, index: usize) -> &Self::Output {
        &self.states[index]
    }
}

impl IndexMut<usize> for States {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.states[index]
    }
}

fn insert_explicit_concat_operator(pattern: &str) -> String {
    let mut output = String::new();
    let mut prev_char: Option<char> = None;

    for token in pattern.chars() {
        if let Some(prev) = prev_char {
            if !(prev == '(' || prev == '|')
                && !(token == '*' || token == '?' || token == '+' || token == '|' || token == ')')
            {
                output.push('\0');
            }
        }
        output.push(token);
        prev_char = Some(token);
    }

    output
}

fn to_postfix(pattern: &str) -> String {
    let mut output = String::new();
    let mut operator_stack = Vec::new();
    let operator_precedence = [('|', 0), ('\0', 1), ('?', 2), ('*', 2), ('+', 2)]
        .iter()
        .cloned()
        .collect::<HashMap<char, i32>>();

    for token in pattern.chars() {
        // Shunting-yard algorithm
        if token == '(' {
            operator_stack.push(token);
        } else if token == ')' {
            while operator_stack.last().map(|&c| c != '(').unwrap_or(false) {
                output.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop();
        } else if token == '\0' || token == '|' || token == '*' || token == '?' || token == '+' {
            while operator_stack.last().is_some()
                && *operator_stack.last().unwrap() != '('
                && operator_precedence[operator_stack.last().unwrap()]
                    >= operator_precedence[&token]
            {
                output.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(token);
        } else {
            output.push(token);
        }
    }

    while !operator_stack.is_empty() {
        output.push(operator_stack.pop().unwrap());
    }

    output
}

#[derive(Debug)]
struct Nfa {
    states: States,
    start: usize,
}

impl Nfa {
    pub fn new(pattern: &str) -> Self {
        let mut states = States::new();
        let f = states.parse(pattern);
        Self {
            states,
            start: f.start,
        }
    }

    pub fn matches(&self, s: &str) -> bool {
        self.states.matches(self.start, s)
    }
}

pub fn is_match(input: &str, pattern: &str) -> bool {
    let nfa = Nfa::new(pattern);
    nfa.matches(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_a() {
        let nfa = Nfa::new("a");
        assert!(nfa.matches("a"));
        assert!(!nfa.matches("ab"));
        assert!(!nfa.matches("aa"));
        assert!(!nfa.matches("b"));
    }

    #[test]
    fn test_pattern_a_or_b() {
        let nfa = Nfa::new("a|b");
        assert!(nfa.matches("a"));
        assert!(nfa.matches("b"));
        assert!(!nfa.matches("ab"));
        assert!(!nfa.matches("aa"));
        assert!(!nfa.matches("ba"));
    }

    #[test]
    fn test_pattern_debug() {
        let nfa = Nfa::new("(a|b)*");
        assert!(nfa.matches("a"));
        assert!(nfa.matches("b"));
        assert!(nfa.matches("aa"));
        assert!(nfa.matches("bb"));
        assert!(nfa.matches("ab"));
        assert!(nfa.matches("ba"));
        assert!(nfa.matches("aaa"));
        assert!(nfa.matches("aba"));
    }

    #[test]
    fn test_pattern_complex() {
        let nfa = Nfa::new("a(b|c)*d");
        assert!(nfa.matches("ad"));
        assert!(nfa.matches("abd"));
        assert!(nfa.matches("acd"));
        assert!(nfa.matches("abbd"));
        assert!(nfa.matches("abcd"));
        assert!(nfa.matches("accd"));
        assert!(nfa.matches("acbd"));
        assert!(nfa.matches("abbbd"));
        assert!(nfa.matches("acccd"));
        assert!(!nfa.matches("a"));
        assert!(!nfa.matches("b"));
        assert!(!nfa.matches("c"));
        assert!(!nfa.matches("d"));
        assert!(!nfa.matches("ab"));
        assert!(!nfa.matches("ac"));
        assert!(!nfa.matches("aad"));
    }

    #[test]
    fn test_fragment_concat_ab() {
        let mut states = States::new();
        let f1 = states.symbol('a');
        let f2 = states.symbol('b');
        let f3 = states.concat(f1, f2);

        assert!(states.matches(f3.start, "ab"));
        assert!(!states.matches(f3.start, "aba"));
        assert!(!states.matches(f3.start, "a"));
        assert!(!states.matches(f3.start, "b"));
    }

    #[test]
    fn test_fragment_union_ab() {
        let mut states = States::new();
        let f1 = states.symbol('a');
        let f2 = states.symbol('b');
        let f3 = states.union(f1, f2);

        assert!(!states.matches(f3.start, "ab"));
        assert!(!states.matches(f3.start, "aba"));
        assert!(states.matches(f3.start, "a"));
        assert!(states.matches(f3.start, "b"));
    }
}
