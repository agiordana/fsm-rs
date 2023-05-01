use std::fmt::Display;
use std::io::Write;

use crate::alphabet::Alphabet;
use crate::nfa::Nfa;

pub fn render_nfa<A: Alphabet>(nfa: &Nfa<A>) -> String
where
    A: Display,
{
    let mut out = Vec::new();
    render_nfa_to(nfa, &mut out).unwrap();
    String::from_utf8(out).unwrap()
}

pub fn render_nfa_to<A: Alphabet, W: Write>(nfa: &Nfa<A>, out: &mut W) -> std::io::Result<()>
where
    A: Display,
{
    writeln!(out, "digraph {{")?;
    writeln!(out, "  rankdir=LR;")?;
    writeln!(out, "  // States: {}", nfa.num_states())?;
    for state in nfa.states() {
        writeln!(
            out,
            "  {} [shape={}];",
            state.id,
            if state.accepting {
                "doublecircle"
            } else {
                "circle"
            }
        )?;
    }
    if !nfa.states.is_empty() {
        writeln!(out, "  // Initial state:")?;
        writeln!(out, "  start [shape=point, color=black];")?;
        writeln!(out, "  start -> 0;")?;
    }
    writeln!(out, "  // Transitions: {}", nfa.num_transitions())?;
    for (from, symbol, to) in nfa.transitions() {
        writeln!(out, "  {} -> {} [label=\"{}\"];", from.id, to.id, symbol)?;
    }
    writeln!(out, "  // ε-Transitions: {}", nfa.num_epsilon_transitions())?;
    for (from, to) in nfa.epsilon_transitions() {
        writeln!(out, "  {} -> {} [label=\"ε\"];", from.id, to.id)?;
    }
    write!(out, "}}")?;
    Ok(())
}
