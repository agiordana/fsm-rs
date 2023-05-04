use fsm_acceptor::dfa::Dfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut dfa = Dfa::new();
    let q0 = dfa.add_state(false);
    let q1 = dfa.add_state(true);
    let _s = dfa.add_state(false);
    dfa.add_transition(q0, 'a', q1);
    dfa.add_transition(q0, 'b', q0);
    dfa.add_transition(q1, 'b', q0);
    dfa.add_transition(q1, 'a', q1);

    println!("dfa = {:?}", dfa);

    let json = serde_json::to_string_pretty(&dfa)?;
    println!("json = {}", json);

    let dfa2: Dfa<char> = serde_json::from_str(&json)?;
    println!("dfa2 = {:?}", dfa2);

    let json3 = "{\"states\": [
        {\"id\": 10, \"accepting\": true, \"transitions\": {\"b\": 10, \"a\": 42}},
        {\"id\": 42, \"accepting\": false, \"transitions\": {\"a\": 42, \"b\": 10}}
    ]}";
    println!("json3 = {}", json3);

    let dfa3: Dfa<char> = serde_json::from_str(&json3)?;
    println!("dfa3 = {:?}", dfa3);
    println!("{}", fsm_acceptor::dfa::graphviz::render_dfa(&dfa3));

    Ok(())
}
