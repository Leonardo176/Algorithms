use crate::{
    Bst,
    trees::{
        bst::node::{Link, Node},
        bstnode::BstSearchNode,
    },
};

// Algoritmi di ricerca e di visita
impl<T: Ord + Clone> Bst<T> {
    pub fn find_node(&self, key: &T) -> Link<T> {
        Node::find(self.root.clone(), key)
    }

    pub fn search(&self, key: T) -> bool {
        if let Some(_) = self.find_node(&key) {
            true
        } else {
            false
        }
    }
}
