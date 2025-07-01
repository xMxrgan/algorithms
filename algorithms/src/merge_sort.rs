pub fn merge_sort(array: &mut [u32]) {
    let length = array.len();

    // Se l'array ha un solo elemento o è vuoto, è già ordinato
    if length <= 1 {
        return;
    }

    // Calcola l'indice centrale per dividere l'array in due metà
    let mid = length / 2;

    // Divide l'array in due metà mutabili
    let (left, right) = array.split_at_mut(mid);

    // Ordina ricorsivamente le due metà
    merge_sort(left);
    merge_sort(right);

    // Crea un nuovo array per contenere i dati uniti
    let mut merged = vec![0; length];
    merge(&mut merged, left, right);

    // Copia il risultato unito nell'array originale
    array.copy_from_slice(&merged);

    // Stampa lo stato dell'array ordinato a questo livello di ricorsione
    println!("{:?}", array);
}

fn merge(result: &mut [u32], left: &[u32], right: &[u32]) {
    let mut l = 0;
    let mut r = 0;
    let mut m = 0;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            result[m] = left[l];
            l += 1;
        } else {
            result[m] = right[r];
            r += 1;
        }
        m += 1;
    }

    while l < left.len() {
        result[m] = left[l];
        l += 1;
        m += 1;
    }

    while r < right.len() {
        result[m] = right[r];
        r += 1;
        m += 1;
    }
}
