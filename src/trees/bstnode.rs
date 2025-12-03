mod base;
// Min/Max/Predecessor/Successor
mod mmps;
mod search;

pub use base::BstBaseNode;
pub use mmps::BstMMPSNode;
pub use search::BstSearchNode;
use std::{cell::RefCell, rc::Rc, rc::Weak};

type Link<T> = Option<Rc<RefCell<T>>>;
type WeakLink<T> = Weak<RefCell<T>>;

/**
 * This is an interface that represents an immutable node of a binary search tree.
 * This is useful because if we have different implementations of a BST (RBTree, standard implementation, etc...)
 * we can use the same algorithms for searching keys on the tree.
 */
pub trait BstNode<T: Ord + Clone>: BstBaseNode<T> + BstMMPSNode<T> + BstSearchNode<T> {}

/**
 * A macro to implement every trait on this module by
 * just specifying the methods of the BstBaseNode trait
 */
#[macro_export]
macro_rules! impl_bstnode {
    ($type:ty, $($it:item),*) => {
    	use crate::trees::bstnode::*;

        impl<T: Ord + Clone> BstBaseNode<T> for $type {
        	$(
         	$it
         	)*
        }

        impl<T: Ord + Clone> BstMMPSNode<T> for $type {}
        impl<T: Ord + Clone> BstSearchNode<T> for $type {}
        impl<T: Ord + Clone> BstNode<T> for $type {}
    };
}
