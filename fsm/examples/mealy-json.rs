use fsm::mealy::Mealy;

fn main() -> Result<(), ()> {
    let mut mealy = Mealy::new();
    let a = mealy.add_state();
    let b = mealy.add_state();
    mealy.add_transition(a, "0", b, "02");
    mealy.add_transition(a, "1", a, "O1");
    mealy.add_transition(a, "2", b, "03");
    mealy.add_transition(b, "0", b, "03");
    mealy.add_transition(b, "1", a, "01");
    mealy.add_transition(b, "2", a, "02");
    println!("mealy = {:?}", mealy);
    let mut current = 0;
    let (next, out) = mealy.next(current,"0").unwrap();
    println!("newstate: {}, output: {}", next, out);
    let json_mealy = serde_json::to_string_pretty(&mealy).unwrap();
    println!("JASON = {}", json_mealy);
    let mealy_json: Mealy<&str,&str> = serde_json::from_str(&json_mealy).unwrap();
    let json_mealy1 = serde_json::to_string_pretty(&mealy_json).unwrap();
    println!("Json1: {}", json_mealy1);
    Ok(())
}
