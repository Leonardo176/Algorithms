use std::rc::Rc;

use super::Link;

// Algoritmi che si possono richiamare su singoli nodi nei BST

pub fn min<T: Ord + Clone>(node: Link<T>) -> Link<T> {
    let mut curr_node = node?;
    let mut data = curr_node.borrow();

    while let Some(next) = data.left.clone() {
        drop(data);
        curr_node = next;
        data = curr_node.borrow();
    }

    drop(data);

    Some(curr_node)
}

pub fn max<T: Ord + Clone>(node: Link<T>) -> Link<T> {
    let mut curr_node = node?;
    let mut data = curr_node.borrow();

    while let Some(next) = data.right.clone() {
        drop(data);
        curr_node = next;
        data = curr_node.borrow();
    }

    drop(data);

    Some(curr_node)
}

pub fn predecessor<T: Ord + Clone>(node: Link<T>) -> Link<T> {
    let mut curr_node = node?;
    let mut data = curr_node.borrow();

    let pred = max(data.left.clone());

    if pred.is_some() {
        pred
    } else {
        // fino a quando c'e' il parent del nodo
        while let Some(parent) = data.parent.upgrade() {
            let parent_data = parent.borrow();

            // se ero il figlio destro, restituisco il parent
            if let Some(dx) = parent_data.right.clone()
                && Rc::ptr_eq(&curr_node, &dx)
            {
                drop(parent_data);
                return Some(parent);
            }

            // altrimenti torno su

            drop(parent_data);
            drop(data);
            curr_node = parent;
            data = curr_node.borrow();
        }

        // se ero sempre a sinistra vuoldire che ero il minimo,
        // quindi il predecessore non esiste

        None
    }
}

pub fn successor<T: Ord + Clone>(node: Link<T>) -> Link<T> {
    let mut curr_node = node?;
    let mut data = curr_node.borrow();

    let pred = min(data.right.clone());

    if pred.is_some() {
        pred
    } else {
        // fino a quando c'e' il parent del nodo
        while let Some(parent) = data.parent.upgrade() {
            let parent_data = parent.borrow();

            // se ero il figlio sinistro, restituisco il parent
            if let Some(sx) = parent_data.left.clone()
                && Rc::ptr_eq(&curr_node, &sx)
            {
                drop(parent_data);
                return Some(parent);
            }

            // altrimenti torno su

            drop(parent_data);
            drop(data);
            curr_node = parent;
            data = curr_node.borrow();
        }

        // se ero sempre a destro vuoldire che ero il massimo,
        // quindi il successore non esiste

        None
    }
}
