mod node;
use node::*;

fn fix_heap(arr: &mut [u64], root: usize, size: usize) {
    let mut curr = root;
    let mut l = left(curr);

    // essendo che l'"albero" è quesicompleto, il vettore non ha figli
    // o ha il figlio sinistro o li ha entrambi
    while l < size {
        let mut i_max = curr;
        if arr[l] > arr[i_max] {
            i_max = l;
        }

        let r = right(curr);

        if r < size && arr[r] > arr[i_max] {
            i_max = r;
        }

        if i_max == curr {
            return;
        }

        swap(arr, i_max, curr);
        curr = i_max;
        l = left(curr);
    }
}

fn heapify(arr: &mut [u64]) {
    let last = arr.len();
    let mut curr = parent(last) + 1;
    while curr > 0 {
        curr -= 1;
        fix_heap(arr, curr, last);
    }
}

pub fn heapsort(arr: &mut [u64]) {
    let len = arr.len();
    heapify(arr);

    let mut i = len;

    while i > 1 {
        i -= 1;

        swap(arr, 0, i);

        fix_heap(arr, 0, i);
    }
}
