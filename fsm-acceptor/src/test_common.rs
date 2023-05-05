use std::collections::VecDeque;
use crate::alphabet::Alphabet;

#[inline]
pub fn decltype<T>(_: &T, x: T) -> T {
    x
}

pub fn generate_words<A:Alphabet>(symbols: &[A], max_length: usize) -> Vec<Vec<A>> {
    let mut words = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(Vec::new());

    while let Some(current) = queue.pop_front() {
        if current.len() <= max_length {
            words.push(current.clone());
            if current.len() < max_length {
                for &symbol in symbols {
                    let mut next = current.clone();
                    next.push(symbol);
                    queue.push_back(next);
                }
            }
        }
    }

    words
}


pub fn generate_strings(symbols: &[char], max_length: usize) -> Vec<String> {
    let mut words = Vec::new();
    let mut queue = VecDeque::from(["".to_string()]);

    while let Some(current) = queue.pop_front() {
        if current.len() <= max_length {
            words.push(current.clone());
            if current.len() < max_length {
                for &symbol in symbols {
                    let mut next = current.clone();
                    next.push(symbol);
                    queue.push_back(next);
                }
            }
        }
    }

    words
}
