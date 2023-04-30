use std::ops::{Index, IndexMut};

pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn alloc(&mut self, item: T) -> usize {
        self.items.push(item);
        self.items.len() - 1
    }

    pub fn alloc_with_id<F>(&mut self, f: F) -> usize
    where
        F: FnOnce(usize) -> T,
    {
        let id = self.items.len();
        self.items.push(f(id));
        id
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Index<usize> for Arena<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for Arena<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
