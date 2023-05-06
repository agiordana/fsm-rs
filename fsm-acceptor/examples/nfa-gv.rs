use fsm_acceptor::nfa::Nfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut nfa = Nfa::new();
    let a = nfa.add_state(false);
    let b = nfa.add_state(true);
    let _s = nfa.add_state(true);
    nfa.add_transition(a, 'a', a);
    nfa.add_transition(a, 'b', a);
    nfa.add_transition(a, 'b', b);
    nfa.add_epsilon_transition(b, b);
    // println!("nfa = {:?}", nfa);
    println!("{}", nfa.render_graphviz());

    Ok(())
}
