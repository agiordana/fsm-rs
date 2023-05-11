use std::fmt::Display;

use graphviz_rust::dot_generator::{attr, edge, graph, id, node, node_id, stmt};
use graphviz_rust::dot_structures::{
    Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Vertex,
};
use graphviz_rust::printer::{DotPrinter, PrinterContext};

use crate::alphabet::Alphabet;
use crate::nfa::Nfa;

impl<A: Alphabet + Display> Nfa<A> {
    pub fn render_graphviz(&self) -> String {
        let mut stmts = Vec::new();

        stmts.push(stmt!(attr!("rankdir", "LR")));

        // States:
        for state in self.states() {
            let name = format!("{}", state.id);
            let attr = attr!(
                "shape",
                if state.accepting {
                    "doublecircle"
                } else {
                    "circle"
                }
            );
            let node = node!( name; attr );
            stmts.push(stmt!(node));
        }

        // Initial state:
        if !self.states.is_empty() {
            let attrs = vec![attr!("shape", "point"), attr!("width", "0")];
            let node = node!("start", attrs);
            stmts.push(stmt!(node));
            let edge = edge!( node_id!("start") => node_id!(0) );
            stmts.push(stmt!(edge));
        }

        // Transitions:
        for (from, symbol, to) in self.transitions() {
            let from = format!("{}", from.id);
            let to = format!("{}", to.id);
            let symbol = format!("{}", symbol);
            let attr = attr!("label", symbol);
            let edge = edge!( node_id!(from) => node_id!(to); attr );
            stmts.push(stmt!(edge));
        }

        // ε-Transitions:
        for (from, to) in self.epsilon_transitions() {
            let from = format!("{}", from.id);
            let to = format!("{}", to.id);
            let attr = attr!("label", esc "ε");
            let edge = edge!( node_id!(from) => node_id!(to); attr );
            stmts.push(stmt!(edge));
        }

        let g = graph!( di id!("NFA"), stmts );
        let mut ctx = PrinterContext::default();
        ctx.with_semi();
        g.print(&mut ctx)
    }

    // pub fn render_graphviz(&self) -> String {
    //     let mut out = Vec::new();
    //     self.render_graphviz_to(&mut out).unwrap();
    //     String::from_utf8(out).unwrap()
    // }
    //
    // pub fn render_graphviz_to<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
    //     writeln!(out, "digraph {{")?;
    //     writeln!(out, "  rankdir=LR;")?;
    //     writeln!(out, "  // States: {}", self.num_states())?;
    //     for state in self.states() {
    //         writeln!(
    //             out,
    //             "  {} [shape={}];",
    //             state.id,
    //             if state.accepting {
    //                 "doublecircle"
    //             } else {
    //                 "circle"
    //             }
    //         )?;
    //     }
    //     if !self.states.is_empty() {
    //         writeln!(out, "  // Initial state:")?;
    //         writeln!(out, "  start [shape=point, color=black];")?;
    //         writeln!(out, "  start -> 0;")?;
    //     }
    //     writeln!(out, "  // Transitions: {}", self.num_transitions())?;
    //     for (from, symbol, to) in self.transitions() {
    //         writeln!(out, "  {} -> {} [label=\"{}\"];", from.id, to.id, symbol)?;
    //     }
    //     writeln!(
    //         out,
    //         "  // ε-Transitions: {}",
    //         self.num_epsilon_transitions()
    //     )?;
    //     for (from, to) in self.epsilon_transitions() {
    //         writeln!(out, "  {} -> {} [label=\"ε\"];", from.id, to.id)?;
    //     }
    //     write!(out, "}}")?;
    //     Ok(())
    // }
}
