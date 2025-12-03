mod node;

use crate::trees::bstnode::*;
use node::{Link, Node};

pub struct Rbt<T: Ord + Clone> {
    root: Link<T>,
}

impl<T: Ord + Clone> Rbt<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, key: &T) -> bool {
        if let Some(_) = Node::find(self.root.clone(), key) {
            true
        } else {
            false
        }
    }

    pub fn in_order(&self) -> Vec<T> {
        Node::in_order(self.root.clone())
    }
}
