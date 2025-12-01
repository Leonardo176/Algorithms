mod sorts;
mod trees;

use crate::max::*;
use rand::random;
use std::iter::repeat_with;

const N: usize = 1_000_000;

#[test]
fn max_test() {
    let v: Vec<u32> = repeat_with(random).take(N).collect();
    assert_eq!(max_2(&v), max_2_opt(&v));
}
