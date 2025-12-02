use super::{Link, WeakLink};

pub trait BstBaseNode<T: Ord + Clone> {
    fn key(&self) -> &T;

    fn left(&self) -> &Link<Self>;
    fn right(&self) -> &Link<Self>;

    fn parent(&self) -> &WeakLink<Self>;
}
