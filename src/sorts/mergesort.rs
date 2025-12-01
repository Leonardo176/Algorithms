/**
 * Merge two sorted arrays (v[start..=mid] and v[mid+1..=end]) into one sorted (v[start..=end])
 *
*/
fn merge(v: &mut Vec<u32>, start: usize, mid: usize, end: usize) {
    let mut aux = Vec::<u32>::with_capacity(end + 1 - start);
    let mut i1 = start;
    let mut i2 = mid + 1;

    while i1 <= mid && i2 <= end {
        if v[i1] < v[i2] {
            aux.push(v[i1]);
            i1 += 1;
        } else {
            aux.push(v[i2]);
            i2 += 1;
        }
    }

    for i in i1..=mid {
        aux.push(v[i]);
    }

    for i in i2..=end {
        aux.push(v[i]);
    }

    for (i, &n) in aux.iter().enumerate() {
        v[start + i] = n;
    }

    // println!("{:?}", aux);
}

/*
 * Sort the array from start index to end index
 */
fn mergesort_ric(v: &mut Vec<u32>, start: usize, end: usize) {
    if start < end {
        let mid = (start + end) / 2;
        mergesort_ric(v, start, mid);
        mergesort_ric(v, mid + 1, end);
        merge(v, start, mid, end);
    }
}

pub fn mergesort(v: &mut Vec<u32>) {
    mergesort_ric(v, 0, v.len() - 1);
}
