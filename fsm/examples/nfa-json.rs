use fsm::nfa::Nfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut nfa = Nfa::new();
    let a = nfa.add_state(false);
    let b = nfa.add_state(true);
    let _s = nfa.add_state(false);
    nfa.add_transition(a, 'a', b);
    nfa.add_transition(a, 'a', a);
    nfa.add_transition(a, 'b', a);
    nfa.add_transition(b, 'a', b);
    nfa.add_transition(b, 'b', b);
    nfa.add_transition(b, 'b', a);

    println!("nfa = {:?}", nfa);

    let json = serde_json::to_string_pretty(&nfa)?;
    println!("json = {}", json);

    let nfa2: Nfa<char> = serde_json::from_str(&json)?;
    println!("nfa2 = {:?}", nfa2);

    let json3 = "{\"states\": [
        {\"id\": 42, \"accepting\": true,
         \"transitions\": {\"b\": [10], \"a\": [42]},
         \"epsilon_transitions\": [10]},
        {\"id\": 10, \"accepting\": false,
         \"transitions\": {\"a\": [42], \"b\": [10]},
         \"epsilon_transitions\": []}
    ]}";
    println!("json3 = {}", json3);

    let nfa3: Nfa<char> = serde_json::from_str(&json3)?;
    println!("nfa3 = {:?}", nfa3);
    println!("{}", nfa3.render_graphviz());

    Ok(())
}
