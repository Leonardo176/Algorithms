use super::N;
use crate::sorts::*;
use rand::{random, random_range};
use std::iter::{repeat_n, repeat_with};

#[test]
fn msort() {
    let mut v: Vec<u32> = repeat_with(random).take(N).collect();
    let mut v_sorted = v.clone();
    mergesort(&mut v);
    v_sorted.sort();
    assert_eq!(v, v_sorted);
}

#[test]
fn hsort() {
    let mut v: Vec<u64> = repeat_with(random).take(N).collect();
    let mut v_sorted = v.clone();
    heapsort(&mut v);
    v_sorted.sort();
    assert_eq!(v, v_sorted);
}

#[test]
fn csort() {
    let k = random_range(1..=1000);
    let v: Vec<u64> = repeat_with(|| random_range(0..=k)).take(N).collect();
    let mut v_mut: Vec<u64> = repeat_n(0, N).collect();
    let mut v_sorted = v.clone();

    counting_sort(&v, &mut v_mut, k as usize);

    v_sorted.sort();
    assert_eq!(v_mut, v_sorted);
}

#[test]
fn qsort() {
    let mut v: Vec<u64> = repeat_with(random).take(N).collect();
    let mut v_sorted = v.clone();
    quicksort(&mut v);
    v_sorted.sort();
    assert_eq!(v, v_sorted);
}
