mod challenge;
mod max;
mod sorts;
mod trees;

pub use max::*;
pub use sorts::*;
pub use trees::Bst;

#[cfg(test)]
mod tests;

fn main() {
    let mut bst: Bst<u64> = Bst::new();

    bst.insert(34);
    bst.insert(5437);
    bst.insert(23);
    bst.insert(124);
    bst.insert(256);
    bst.insert(34567875);

    bst.delete(34);
    bst.delete(5437);
    bst.insert(400);
    bst.delete(23);
    bst.insert(0);
    bst.insert(1);
    bst.delete(124);
    bst.delete(400);
    bst.delete(256);
    bst.insert(5);
    bst.delete(34567875);

    println!("{:?}", bst.in_order());
}
