use super::{BstBaseNode, BstMMPSNode, Link};

pub trait BstSearchNode<T: Ord + Clone>: BstBaseNode<T> + BstMMPSNode<T> {
    fn find(node: Link<Self>, key: &T) -> Link<Self> {
        let mut curr_node = node?;
        let mut data = curr_node.borrow();

        while let Some(next) = {
            if key < data.key() {
                data.left().clone()
            } else if key > data.key() {
                data.right().clone()
            } else {
                drop(data);
                return Some(curr_node);
            }
        } {
            drop(data);
            curr_node = next;
            data = curr_node.borrow();
        }
        None
    }

    fn in_order(node: Link<Self>) -> Vec<T> {
        let mut vec = Vec::new();

        let mut curr = Self::min(node);

        while let Some(next) = curr.take() {
            vec.push(next.borrow().key().clone());
            curr = Self::successor(Some(next));
        }

        vec
    }
}
