use std::fmt::Display;
use std::io::Write;

use crate::alphabet::Alphabet;
use crate::dfa::Dfa;

pub fn render_dfa<A: Alphabet>(dfa: &Dfa<A>) -> String
where
    A: Display,
{
    let mut out = Vec::new();
    render_dfa_to(dfa, &mut out).unwrap();
    String::from_utf8(out).unwrap()
}

pub fn render_dfa_to<A: Alphabet, W: Write>(dfa: &Dfa<A>, out: &mut W) -> std::io::Result<()>
where
    A: Display,
{
    writeln!(out, "digraph {{")?;
    writeln!(out, "  rankdir=LR;")?;
    writeln!(out, "  // States: {}", dfa.num_states())?;
    for state in dfa.states() {
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
    if !dfa.states.is_empty() {
        writeln!(out, "  // Initial state:")?;
        writeln!(out, "  start [shape=point, color=black];")?;
        writeln!(out, "  start -> 0;")?;
    }
    writeln!(out, "  // Transitions: {}", dfa.num_transitions())?;
    for (from, symbol, to) in dfa.transitions() {
        writeln!(out, "  {} -> {} [label=\"{}\"];", from.id, to.id, symbol)?;
    }
    write!(out, "}}")?;
    Ok(())
}
