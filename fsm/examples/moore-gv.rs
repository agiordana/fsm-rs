use fsm::moore::Moore;

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

    Ok(())
}
