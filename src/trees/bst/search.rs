use crate::{Bst, trees::node::Link};

// Algoritmi di ricerca e di visita
impl<T: Ord + Clone> Bst<T> {
    pub fn find_node(&self, key: &T) -> Link<T> {
        let mut curr = self.root.clone()?;

        let mut data = curr.borrow();

        while let Some(next) = if *key < data.key {
            data.left.clone()
        } else if *key > data.key {
            data.right.clone()
        } else {
            return Some(curr.clone());
        } {
            drop(data);
            curr = next;
            data = curr.borrow();
        }
        None
    }

    pub fn search(&self, key: T) -> bool {
        if let Some(_) = self.find_node(&key) {
            true
        } else {
            false
        }
    }
}
