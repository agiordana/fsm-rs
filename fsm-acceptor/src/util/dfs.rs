use std::collections::HashSet;
use std::hash::Hash;

pub fn dfs<T, F, I>(start: T, neighbors: F) -> Dfs<T, F>
where
    T: Hash + Eq + Copy,
    F: Fn(T) -> I,
    I: IntoIterator<Item = T>,
{
    Dfs::new(vec![start], neighbors)
}

pub fn multi_dfs<T, F, I>(start: Vec<T>, neighbors: F) -> Dfs<T, F>
where
    T: Hash + Eq + Copy,
    F: Fn(T) -> I,
    I: IntoIterator<Item = T>,
{
    Dfs::new(start, neighbors)
}

#[derive(Debug)]
pub struct Dfs<T, F> {
    stack: Vec<T>,
    visited: HashSet<T>,
    neighbors: F,
}

impl<T, F, I> Dfs<T, F>
where
    F: Fn(T) -> I,
    I: IntoIterator<Item = T>,
{
    pub fn new(stack: Vec<T>, neighbors: F) -> Self {
        Dfs {
            stack,
            visited: HashSet::new(),
            neighbors,
        }
    }
}

impl<T, F, I> Iterator for Dfs<T, F>
where
    T: Hash + Eq + Copy,
    F: Fn(T) -> I,
    I: IntoIterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.stack.pop() {
            if self.visited.insert(state) {
                for next_state in (self.neighbors)(state) {
                    self.stack.push(next_state);
                }
                return Some(state);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let neighbors = |state: u32| match state {
            0 => vec![1, 2],
            1 => vec![3],
            2 => vec![1, 4],
            3 => vec![5],
            4 => vec![1, 5],
            5 => vec![],
            _ => unreachable!(),
        };
        // for state in dfs(0, neighbors) {
        //     println!("state: {}", state);
        // }
        let mut traversal = dfs(0, neighbors);
        assert_eq!(traversal.next(), Some(0));
        assert_eq!(traversal.next(), Some(2));
        assert_eq!(traversal.next(), Some(4));
        assert_eq!(traversal.next(), Some(5));
        assert_eq!(traversal.next(), Some(1));
        assert_eq!(traversal.next(), Some(3));
        assert_eq!(traversal.next(), None);
    }
}
