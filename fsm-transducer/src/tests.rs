use crate::fsm::FSM;
use crate::guard::Guard;
use crate::output::OutputEvent;
use crate::state::State;
use crate::symbol::Symbol;
use crate::transition::Transition;

#[test]
fn test_fsm() {
    let mut fsm = FSM::new();
    assert_eq!(fsm.states.len(), 0);

    let mut state = State::new(0, OutputEvent::INIT);
    let transition = Transition::new(0, 1, Guard::new(Symbol::A));
    state.add_transition(transition);
    fsm.add_state(state);
    assert_eq!(fsm.states.len(), 1);

    let state = fsm.state(0).unwrap();
    assert_eq!(state.id, 0);
    assert_eq!(state.event, OutputEvent::INIT);
    assert_eq!(state.transitions.len(), 1);

    let transition = &state.transitions[0];
    assert_eq!(transition.source, 0);
    assert_eq!(transition.destination, 1);
    assert_eq!(transition.guard.input, Symbol::A);

    let mut state = State::new(1, OutputEvent::CNF);
    let transition = Transition::new(1, 2, Guard::new(Symbol::B));
    state.add_transition(transition);
    fsm.add_state(state);
    assert_eq!(fsm.states.len(), 2);
}
