use super::bstnode::BstNode;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type WeakLink<T> = Weak<RefCell<Node<T>>>;

pub struct Node<T: Ord + Clone> {
    pub key: T,
    pub left: Link<T>,
    pub right: Link<T>,
    pub parent: WeakLink<T>,
}

impl<T: Ord + Clone> Node<T> {
    pub fn new(key: T) -> Self {
        Node {
            key,
            left: None,
            right: None,
            parent: Weak::new(),
        }
    }
}

impl<T: Ord + Clone> BstNode<T> for Node<T> {
    fn key(&self) -> &T {
        &self.key
    }

    fn left(&self) -> &Link<T> {
        &self.left
    }

    fn right(&self) -> &Link<T> {
        &self.right
    }

    fn parent(&self) -> &WeakLink<T> {
        &self.parent
    }
}

pub fn link_new<T: Ord + Clone>(key: T) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(Node::new(key)))
}
