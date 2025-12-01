// Input:  Vettore A di caratteri, indice I
// spostare A[i..] davanti
// Esempio: Input: "ciao_leonardo", 5 -> "leonardociao_"
// "ciao_leonardo" -> "nardoleociao_" -> "odranoelciao_" -> "leonardociao_"
// "papleo", 3 -> "leopap"
// "barcosup", 5 -> "supocrab" -> "supbarco"
// Soluzione in Place e O(n)
pub fn shift(v: &mut Vec<char>, i: usize) {
    let len = v.len();
    reverse(v, 0, i - 1);
    reverse(v, i, len - 1);
    reverse(v, 0, len - 1);
}

fn swap(v: &mut Vec<char>, i1: usize, i2: usize) {
    let aux = v[i1];
    v[i1] = v[i2];
    v[i2] = aux;
}

fn reverse(v: &mut Vec<char>, i1: usize, i2: usize) {
    // per preservare l'immutabilita' di i1 e i2
    if i1 > i2 {
        reverse(v, i2, i1);
    }

    for i in 0..(i2 - i1 + 1) / 2 {
        swap(v, i1 + i, i2 - i);
    }
}
