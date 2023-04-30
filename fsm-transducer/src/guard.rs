use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guard {
    pub input: Symbol,
}

impl Guard {
    pub fn new(input: Symbol) -> Self {
        Guard { input }
    }

    pub fn matches(&self, input: Symbol) -> bool {
        self.input == input
    }
}
