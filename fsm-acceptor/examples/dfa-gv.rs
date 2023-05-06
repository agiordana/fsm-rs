use fsm_acceptor::dfa::Dfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut dfa = Dfa::new();
    let a = dfa.add_state(false);
    let b = dfa.add_state(true);
    let _s = dfa.add_state(true);
    dfa.add_transition(a, 'a', b);
    dfa.add_transition(a, 'b', a);
    dfa.add_transition(b, 'b', a);
    dfa.add_transition(b, 'a', b);
    // println!("dfa = {:?}", dfa);
    println!("{}", dfa.render_graphviz());

    Ok(())
}
