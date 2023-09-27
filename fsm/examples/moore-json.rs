use fsm::moore::Moore;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), ()> {
    let mut moore = Moore::new();
    let a = moore.add_state("00");
    let b = moore.add_state("01");
    let c = moore.add_state("11");
    let d = moore.add_state("10");
    moore.add_transition(a, "click", b);
    moore.add_transition(a, "l1=0", a);
    moore.add_transition(a, "l2=0", a);
    moore.add_transition(b, "click", c);
    moore.add_transition(b, "l2=0", a);
    moore.add_transition(b, "l1=0", b);
    moore.add_transition(b, "l2=1", b);
    moore.add_transition(b, "l1=1", c);
    moore.add_transition(c, "click", d);
    moore.add_transition(c, "l1=0", b);
    moore.add_transition(c, "l1=1", c);
    moore.add_transition(c, "l2=0", d);
    moore.add_transition(c, "l2=1", c);
    moore.add_transition(d, "click", a);
    moore.add_transition(d, "l1=0", a);
    moore.add_transition(d, "l1=1", d);
    moore.add_transition(d, "l2=0", d);
    moore.add_transition(d, "l2=1", c);
    println!("moore = {:?}", moore);
    let mut current = 0;
    let (next, out) = moore.next(current,"click").unwrap();
    println!("newstate: {}, output: {}", next, out);
    let json_moore = serde_json::to_string_pretty(&moore).unwrap();
    println!("Json: {}", json_moore);
    let moore_json: Moore<&str,&str> = serde_json::from_str(&json_moore).unwrap();
    println!("moore_json = {:?}", moore_json);
    let json_moore1 = serde_json::to_string_pretty(&moore_json).unwrap();
    println!("Json1: {}", json_moore1);
    Ok(())
}
