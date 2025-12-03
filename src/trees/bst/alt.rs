use std::{cell::RefCell, mem::swap, rc::Rc};

use crate::{
    Bst,
    trees::{
        bst::node::{Link, Node},
        bstnode::BstMMPSNode,
    },
};

// Implementazioni alternative per gli stessi algoritmo
impl<T: Ord + Clone> Bst<T> {
    pub fn _insert_2(&mut self, key: T) {
        if let Some(root) = &self.root {
            let mut curr_node = root.clone();

            loop {
                let mut data = curr_node.borrow_mut();
                if key < data.key {
                    match data.left.clone() {
                        Some(node) => {
                            drop(data);
                            curr_node = node;
                        }
                        None => {
                            let mut node = Node::new(key);
                            node.parent = Rc::downgrade(&curr_node);
                            data.left = Some(Rc::new(RefCell::from(node)));

                            return;
                        }
                    }
                } else if key > data.key {
                    match data.right.clone() {
                        Some(node) => {
                            drop(data);
                            curr_node = node;
                        }
                        None => {
                            let mut node = Node::new(key);
                            node.parent = Rc::downgrade(&curr_node);
                            data.right = Some(Rc::new(RefCell::from(node)));

                            return;
                        }
                    }
                } else {
                    panic!("Key already present!");
                }
            }
        } else {
            self.root = Some(Rc::new(RefCell::from(Node::new(key))));
        }
    }

    pub fn _in_order_2(&self) -> Vec<T> {
        let mut arr = Vec::new();

        let mut next = self.root.clone();

        let mut prec: Link<T> = None;

        while let Some(curr_node) = next.take() {
            let data = curr_node.borrow();
            let mut go_up = false;

            match prec.take() {
                None => {
                    if let Some(sx) = data.left.clone() {
                        next = Some(sx);
                    } else {
                        arr.push(data.key.clone());
                        if let Some(dx) = data.right.clone() {
                            next = Some(dx);
                        } else {
                            go_up = true;
                        }
                    }
                }
                Some(node) => {
                    if let Some(sx) = data.left.clone()
                        && Rc::ptr_eq(&sx, &node)
                    {
                        arr.push(data.key.clone());
                        if let Some(dx) = data.right.clone() {
                            next = Some(dx);
                        } else {
                            go_up = true;
                        }
                    } else {
                        go_up = true;
                    }
                }
            }

            if go_up {
                next = data.parent.upgrade();
                drop(data);
                prec = Some(curr_node);
            }
        }

        arr
    }

    pub fn delete_2(&mut self, key: T) {
        let node = self.find_node(&key);

        if let Some(node) = node {
            let data = node.borrow();
            if data.left.is_none() || data.right.is_none() {
                // Caso facile: uno dei due figli del nodo da eliminare e' None

                // Chiamiamo branch il ramo che ha i figli
                let branch = if let Some(node) = data.left.clone() {
                    Some(node)
                } else if let Some(node) = data.right.clone() {
                    Some(node)
                } else {
                    None
                };

                if let Some(parent) = data.parent.upgrade() {
                    // Aggancia il parent del ramo da spostare in alto
                    if let Some(branch) = branch.clone() {
                        let mut data = branch.borrow_mut();
                        data.parent = Rc::downgrade(&parent);
                    }

                    let mut parent_data = parent.borrow_mut();

                    if let Some(sx) = &parent_data.left
                        && Rc::ptr_eq(sx, &node)
                    {
                        parent_data.left = branch;
                    } else if let Some(dx) = &parent_data.right
                        && Rc::ptr_eq(dx, &node)
                    {
                        parent_data.right = branch;
                    }
                } else {
                    self.root = branch;
                }
            } else {
                // Caso non semplice: il nodo ha due figli
                // Troviamo il suo successore, che esiste sempre perche' ha il figlio destro

                drop(data);
                let succ = Node::successor(Some(node.clone())).unwrap();
                let mut data = node.borrow_mut();
                let mut data_succ = succ.borrow_mut();

                swap(&mut data.key, &mut data_succ.key);

                drop(data);

                let branch = if let Some(node) = data_succ.left.clone() {
                    Some(node)
                } else if let Some(node) = data_succ.right.clone() {
                    Some(node)
                } else {
                    None
                };

                let parent = data_succ.parent.upgrade().unwrap();
                let mut parent_data = parent.borrow_mut();

                if let Some(branch) = branch.clone() {
                    let mut data = branch.borrow_mut();
                    data.parent = Rc::downgrade(&parent);
                }

                if let Some(sx) = &parent_data.left
                    && Rc::ptr_eq(&succ, sx)
                {
                    parent_data.left = branch;
                } else if let Some(dx) = &parent_data.right
                    && Rc::ptr_eq(&succ, dx)
                {
                    parent_data.right = branch;
                }
            }
        }
    }
}
