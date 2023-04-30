use std::fmt::Display;

use crate::alphabet::Alphabet;
use crate::nfa::Nfa;

pub fn nfa_to_graphviz<A: Alphabet>(nfa: &Nfa<A>) -> String
where
    A: Display,
{
    let mut out = String::new();
    out += "digraph {\n";
    out += "  rankdir=LR;\n";
    out += &format!("  // States ({}):\n", nfa.num_states());
    for state in nfa.states() {
        out += &format!(
            "  {} [shape={}];\n",
            state.id,
            if state.accepting {
                "doublecircle"
            } else {
                "circle"
            }
        );
    }
    if !nfa.states.is_empty() {
        out += "  // Initial state:\n";
        out += "  start [shape=point, color=black];\n";
        out += &format!("  start -> 0;\n");
    }
    out += &format!("  // Transitions ({}):\n", nfa.num_transitions());
    for (from, symbol, to) in nfa.transitions() {
        out += &format!("  {} -> {} [label=\"{}\"];\n", from.id, to.id, symbol);
    }
    out += &format!("  // ε-Transitions ({}):\n", nfa.num_epsilon_transitions());
    for (from, to) in nfa.epsilon_transitions() {
        out += &format!("  {} -> {} [label=\"ε\"];\n", from.id, to.id);
    }
    out += "}";
    out
}
