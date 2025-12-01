fn swap(arr: &mut [u64], i: usize, j: usize) {
    let aux = arr[i];
    arr[i] = arr[j];
    arr[j] = aux;
}

pub fn partition(arr: &mut [u64], start: usize, end: usize) -> usize {
    let pivot = arr[end];
    let mut max_low = start;

    for i in start..end {
        if arr[i] <= pivot {
            swap(arr, max_low, i);
            max_low += 1;
        }
    }

    swap(arr, max_low, end);

    max_low
}

fn quicksort_ric(arr: &mut [u64], start: usize, end: usize) {
    if start < end {
        let pivot = partition(arr, start, end);

        if pivot > 0 {
            quicksort_ric(arr, start, pivot - 1);
        }

        quicksort_ric(arr, pivot + 1, end);
    }
}

pub fn quicksort(arr: &mut [u64]) {
    quicksort_ric(arr, 0, arr.len() - 1);
}
