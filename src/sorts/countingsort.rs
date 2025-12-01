use std::iter::repeat_n;

/**
 * PRE: forall i in a, 0 <= i <= k
 * 		b must have the same size of a
 * POST:
 * 		b will be modified in order to contain the elements of a sorted
 * 		a is not modified
 */
pub fn counting_sort(a: &[u64], b: &mut [u64], k: usize) {
    let mut c: Vec<u64> = repeat_n(0, k + 1).collect();

    for i in 0..a.len() {
        c[a[i] as usize] += 1;
    }

    for i in 1..=k {
        c[i] = c[i] + c[i - 1];
    }

    for i in (0..a.len()).rev() {
        println!("{}", c[a[i] as usize] as usize);
        b[c[a[i] as usize] as usize - 1] = a[i];

        c[a[i] as usize] -= 1;
    }
}
