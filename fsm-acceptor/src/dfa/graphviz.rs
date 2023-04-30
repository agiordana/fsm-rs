use std::fmt::Display;

use crate::alphabet::Alphabet;
use crate::dfa::Dfa;

pub fn dfa_to_graphviz<A: Alphabet>(dfa: &Dfa<A>) -> String
where
    A: Display,
{
    let mut out = String::new();
    out += "digraph {\n";
    out += "  rankdir=LR;\n";
    out += "  // Nodes:\n";
    for state in dfa.states() {
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
    if !dfa.states.is_empty() {
        out += "  // Initial state:\n";
        out += "  start [shape=point, color=black];\n";
        out += &format!("  start -> 0;\n");
    }
    out += "  // Transitions:\n";
    for (from, symbol, to) in dfa.transitions() {
        out += &format!("  {} -> {} [label=\"{}\"];\n", from.id, to.id, symbol);
    }
    out += "}";
    out
}
