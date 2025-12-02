mod alt;
mod drop;
mod search;

use super::node::*;
use crate::trees::bstnode::BstNode;
use std::{cell::RefCell, mem::swap, rc::Rc};

pub struct Bst<T: Ord + Clone> {
    root: Link<T>,
}

impl<T: Ord + Clone> Bst<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: T) {
        if let Some(root) = &self.root {
            let mut curr_node = root.clone();

            let mut data = curr_node.borrow();

            // se c'e' un nodo successivo dove inserire la chiave
            while let Some(next) = if key < data.key {
                data.left.clone()
            } else if key > data.key {
                data.right.clone()
            } else {
                panic!("Two equal keys on the same BST are not allowed!")
            } {
                // vado avanti
                drop(data);
                curr_node = next;
                data = curr_node.borrow();
            }

            drop(data);

            let mut data = curr_node.borrow_mut();

            let mut node = Node::new(key);
            node.parent = Rc::downgrade(&curr_node);

            if node.key < data.key {
                data.left = Some(Rc::new(RefCell::from(node)));
            } else if node.key > data.key {
                data.right = Some(Rc::new(RefCell::from(node)));
            } else {
                panic!("Two equal keys on the same BST are not allowed!");
            }
        } else {
            self.root = Some(link_new(key));
        }
    }

    pub fn delete(&mut self, key: T) {
        let target = self.find_node(&key);

        if let Some(mut target) = target {
            let data = target.borrow();

            if data.left.is_some() && data.right.is_some() {
                drop(data);

                let succ = Node::successor(Some(target.clone())).unwrap();
                let mut data = target.borrow_mut();
                let mut data_succ = succ.borrow_mut();

                swap(&mut data.key, &mut data_succ.key);

                drop(data);
                drop(data_succ);

                target = succ;
            }

            let data = target.borrow();

            let branch = if data.left.is_some() {
                data.left.clone()
            } else if data.right.is_some() {
                data.right.clone()
            } else {
                None
            };

            if let Some(parent) = data.parent.upgrade() {
                drop(data);

                if let Some(branch) = branch.clone() {
                    let mut data = branch.borrow_mut();
                    data.parent = Rc::downgrade(&parent);
                }

                let mut parent_data = parent.borrow_mut();

                if let Some(sx) = &parent_data.left
                    && Rc::ptr_eq(sx, &target)
                {
                    parent_data.left = branch;
                } else if let Some(dx) = &parent_data.right
                    && Rc::ptr_eq(dx, &target)
                {
                    parent_data.right = branch;
                }
            } else {
                self.root = branch;
            }
        }
    }

    pub fn in_order(&self) -> Vec<T> {
        let mut arr = Vec::new();

        let mut curr = Node::min(self.root.clone());

        while let Some(node) = curr.take() {
            let data = node.borrow();
            arr.push(data.key.clone());
            drop(data);

            curr = Node::successor(Some(node));
        }

        arr
    }
}
