use crate::guard::Guard;
use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub source: usize,
    pub destination: usize,
    pub guard: Guard,
}

impl Transition {
    pub fn new(source: usize, destination: usize, guard: Guard) -> Self {
        Transition {
            source,
            destination,
            guard,
        }
    }

    pub fn next(&self, input: Symbol) -> Option<usize> {
        if self.guard.matches(input) {
            Some(self.destination)
        } else {
            None
        }
    }
}
