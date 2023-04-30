use fsm_acceptor::nfa::graphviz::nfa_to_graphviz;
use fsm_acceptor::nfa::Nfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut nfa = Nfa::new();
    let p = nfa.add_state(false);
    let q = nfa.add_state(true);
    nfa.add_transition(p, 'a', p);
    nfa.add_transition(p, 'b', p);
    nfa.add_transition(p, 'b', q);
    // println!("nfa = {:?}", nfa);
    println!("{}", nfa_to_graphviz(&nfa));

    Ok(())
}
