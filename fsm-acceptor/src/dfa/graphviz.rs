use std::fmt::Display;
use std::io::Write;

use crate::alphabet::Alphabet;
use crate::dfa::Dfa;

impl<A: Alphabet + Display> Dfa<A> {
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
        write!(out, "}}")?;
        Ok(())
    }
}
