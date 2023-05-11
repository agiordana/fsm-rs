use fsm::dfa::Dfa;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut dfa = Dfa::new();
    let a = dfa.add_state(false);
    let b = dfa.add_state(true);
    let _s = dfa.add_state(false);
    dfa.add_transition(a, 'a', b);
    dfa.add_transition(a, 'b', a);
    dfa.add_transition(b, 'b', a);
    dfa.add_transition(b, 'a', b);

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
    println!("{}", dfa3.render_graphviz());

    Ok(())
}
