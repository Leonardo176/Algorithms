use std::{cell::RefCell, rc::Rc, rc::Weak};

type Link<T> = Option<Rc<RefCell<T>>>;
type WeakLink<T> = Weak<RefCell<T>>;

/**
 * This is an interface that represents an immutable node of a binary search tree.
 * This is useful because if we have different implementations of a BST (RBTree, standard implementation, etc...)
 * we can use the same algorithms for searching keys on the tree.
 */
pub trait BstNode<T: Ord + Clone> {
    fn key(&self) -> &T;

    fn left(&self) -> &Link<Self>;
    fn right(&self) -> &Link<Self>;

    fn parent(&self) -> &WeakLink<Self>;

    fn min(node: Link<Self>) -> Link<Self> {
        let mut curr_node = node?;

        while let Some(next) = { curr_node.borrow().left().clone() } {
            curr_node = next;
        }

        Some(curr_node)
    }

    fn max(node: Link<Self>) -> Link<Self> {
        let mut curr_node = node?;

        while let Some(next) = { curr_node.borrow().right().clone() } {
            curr_node = next;
        }

        Some(curr_node)
    }

    fn predecessor(node: Link<Self>) -> Link<Self> {
        let mut curr_node = node?;
        let mut data = curr_node.borrow();

        let max = Self::max(data.left().clone());
        if max.is_some() {
            return max;
        }

        while let Some(parent) = data.parent().upgrade() {
            if let Some(dx) = { parent.borrow().right().clone() }
                && Rc::ptr_eq(&dx, &curr_node)
            {
                return Some(parent);
            }

            drop(data);
            curr_node = parent;
            data = curr_node.borrow();
        }

        None
    }

    fn successor(node: Link<Self>) -> Link<Self> {
        let mut curr_node = node?;
        let mut data = curr_node.borrow();

        let min = Self::min(data.right().clone());
        if min.is_some() {
            return min;
        }

        while let Some(parent) = data.parent().upgrade() {
            if let Some(sx) = { parent.borrow().left().clone() }
                && Rc::ptr_eq(&sx, &curr_node)
            {
                return Some(parent);
            }

            drop(data);
            curr_node = parent;
            data = curr_node.borrow();
        }

        None
    }
}
