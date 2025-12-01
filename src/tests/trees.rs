use super::N;
use crate::Bst;
use rand::random;
use std::iter::repeat_with;

#[test]
fn bsort() {
    let v: Vec<u64> = repeat_with(random).take(N).collect();
    let mut v_sorted = v.clone();
    let mut bst = Bst::new();

    for n in v {
        bst.insert(n);
    }
    v_sorted.sort();

    let v = bst.in_order();

    assert_eq!(v, v_sorted);
}

#[test]
fn insert_delete() {
    let v: Vec<u64> = repeat_with(random).take(N).collect();
    let mut v_sorted = v.clone();
    let mut bst = Bst::new();

    for n in v {
        bst.insert(n);
    }

    for i in repeat_with(random::<u64>).take(1000) {
        let i = (i % v_sorted.len() as u64) as usize;

        let n = v_sorted.swap_remove(i);

        bst.delete(n);
    }

    v_sorted.sort();

    let v = bst.in_order();

    assert_eq!(v, v_sorted);
}
