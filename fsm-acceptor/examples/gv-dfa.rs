use fsm_acceptor::dfa::graphviz::dfa_to_graphviz;
use fsm_acceptor::dfa::Dfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut dfa = Dfa::new();
    let q0 = dfa.add_state(false);
    let q1 = dfa.add_state(true);
    dfa.add_transition(q0, 'a', q1);
    dfa.add_transition(q1, 'b', q0);
    // println!("dfa = {:?}", dfa);
    println!("{}", dfa_to_graphviz(&dfa));

    Ok(())
}
