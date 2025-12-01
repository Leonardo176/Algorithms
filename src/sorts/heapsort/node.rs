pub fn left(i: usize) -> usize {
    2 * i + 1
}

pub fn right(i: usize) -> usize {
    2 * i + 2
}

pub fn parent(i: usize) -> usize {
    i / 2
}

pub fn swap(arr: &mut [u64], i1: usize, i2: usize) {
    let aux = arr[i1];
    arr[i1] = arr[i2];
    arr[i2] = aux;
}
