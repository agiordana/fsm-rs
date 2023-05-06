use std::collections::HashSet;
use std::hash::Hash;

pub fn dfs<T, F, I>(start: T, neighbors: F) -> impl Iterator<Item = T>
where
    T: Hash + Eq + Copy,
    F: Fn(T) -> I,
    I: IntoIterator<Item = T>,
{
    multi_dfs(vec![start], neighbors)
}

pub fn multi_dfs<T, F, I>(start: Vec<T>, neighbors: F) -> impl Iterator<Item = T>
where
    T: Hash + Eq + Copy,
    F: Fn(T) -> I,
    I: IntoIterator<Item = T>,
{
    let mut visited = HashSet::new();
    let mut stack = start;
    std::iter::from_fn(move || {
        while let Some(state) = stack.pop() {
            if visited.insert(state) {
                stack.extend(neighbors(state));
                return Some(state);
            }
        }
        None
    })
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
