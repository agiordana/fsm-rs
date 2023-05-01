use fsm_acceptor::nfa::graphviz::render_nfa;
use fsm_acceptor::nfa::Nfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut nfa = Nfa::new();
    let p = nfa.add_state(false);
    let q = nfa.add_state(true);
    let _s = nfa.add_state(true);
    nfa.add_transition(p, 'a', p);
    nfa.add_transition(p, 'b', p);
    nfa.add_transition(p, 'b', q);
    nfa.add_epsilon_transition(q, q);
    // println!("nfa = {:?}", nfa);
    println!("{}", render_nfa(&nfa));

    Ok(())
}
