use std::fmt::Display;
use std::io::Write;

use crate::alphabet::Alphabet;
use crate::nfa::Nfa;

impl<A: Alphabet + Display> Nfa<A> {
    pub fn render_graphviz(&self) -> String {
        let mut out = Vec::new();
        self.render_graphviz_to(&mut out).unwrap();
        String::from_utf8(out).unwrap()
    }

    pub fn render_graphviz_to<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        writeln!(out, "digraph {{")?;
        writeln!(out, "  rankdir=LR;")?;
        writeln!(out, "  // States: {}", self.num_states())?;
        for state in self.states() {
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
        if !self.states.is_empty() {
            writeln!(out, "  // Initial state:")?;
            writeln!(out, "  start [shape=point, color=black];")?;
            writeln!(out, "  start -> 0;")?;
        }
        writeln!(out, "  // Transitions: {}", self.num_transitions())?;
        for (from, symbol, to) in self.transitions() {
            writeln!(out, "  {} -> {} [label=\"{}\"];", from.id, to.id, symbol)?;
        }
        writeln!(
            out,
            "  // ε-Transitions: {}",
            self.num_epsilon_transitions()
        )?;
        for (from, to) in self.epsilon_transitions() {
            writeln!(out, "  {} -> {} [label=\"ε\"];", from.id, to.id)?;
        }
        write!(out, "}}")?;
        Ok(())
    }
}
