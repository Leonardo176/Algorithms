use std::{cell::RefCell, rc::Rc, rc::Weak};

use crate::impl_bstnode;

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type WeakLink<T> = Weak<RefCell<Node<T>>>;

enum Color {
    BLACK,
    RED,
}

pub struct Node<T: Ord + Clone> {
    key: T,
    color: Color,
    left: Link<T>,
    right: Link<T>,
    parent: WeakLink<T>,
}

impl_bstnode!(
    Node<T>,
    fn key(&self) -> &T {
        &self.key
    },
    fn left(&self) -> &Link<T> {
        &self.left
    },
    fn right(&self) -> &Link<T> {
        &self.right
    },
    fn parent(&self) -> &WeakLink<T> {
        &self.parent
    }
);
