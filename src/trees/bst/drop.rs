use std::rc::Rc;

use crate::Bst;

impl<T: Ord + Clone> Drop for Bst<T> {
    fn drop(&mut self) {
        if let Some(mut curr) = self.root.take() {
            let mut data = curr.borrow();
            while data.left.is_some() || data.right.is_some() || data.parent.upgrade().is_some() {
                let next;

                if let Some(l) = data.left.clone() {
                    next = l;
                } else if let Some(r) = data.right.clone() {
                    next = r;
                } else {
                    let parent = data.parent.upgrade().unwrap();
                    let mut parent_data = parent.borrow_mut();

                    if let Some(l) = &parent_data.left
                        && Rc::ptr_eq(&l, &curr)
                    {
                        parent_data.left = None;
                    } else {
                        parent_data.right = None;
                    }

                    drop(parent_data);

                    next = parent;
                }

                drop(data);
                curr = next;
                data = curr.borrow();
            }
        }
    }
}
