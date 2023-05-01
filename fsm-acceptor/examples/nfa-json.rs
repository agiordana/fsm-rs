use fsm_acceptor::nfa::Nfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut nfa = Nfa::new();
    let q0 = nfa.add_state(false);
    let q1 = nfa.add_state(true);
    let _s = nfa.add_state(false);
    nfa.add_transition(q0, 'a', q1);
    nfa.add_transition(q0, 'a', q0);
    nfa.add_transition(q0, 'b', q0);
    nfa.add_transition(q1, 'a', q1);
    nfa.add_transition(q1, 'b', q1);
    nfa.add_transition(q1, 'b', q0);

    println!("nfa = {:?}", nfa);

    let json = serde_json::to_string_pretty(&nfa)?;
    println!("json = {}", json);

    let nfa2: Nfa<char> = serde_json::from_str(&json)?;
    println!("nfa2 = {:?}", nfa2);

    let json3 = "{\"states\": [
        {\"id\": 1, \"accepting\": true,
         \"transitions\": {\"b\": [0], \"a\": [1]},
         \"epsilon_transitions\": [0]},
        {\"id\": 0, \"accepting\": false,
         \"transitions\": {\"a\": [1], \"b\": [0]},
         \"epsilon_transitions\": []}
    ]}";
    println!("json3 = {}", json3);

    let nfa3: Nfa<char> = serde_json::from_str(&json3)?;
    println!("nfa3 = {:?}", nfa3);
    println!("{}", fsm_acceptor::nfa::graphviz::render_nfa(&nfa3));

    Ok(())
}
