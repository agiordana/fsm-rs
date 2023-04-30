#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputAction {
    Epsilon,
    Action {
        event: OutputEvent,
        values: Vec<bool>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OutputEvent {
    INIT,
    CNF,
}
