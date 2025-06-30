pub fn merge_sort(arr: &mut [u32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Divide l'array in due metà
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    println!("{:?} – {:?}", left, right);

    // Ordina ricorsivamente ciascuna metà
    merge_sort(&mut left);
    merge_sort(&mut right);

    // Unisci le due metà ordinate
    merge(arr, &left[..], &right[..]);
}

fn merge(result: &mut [u32], left: &[u32], right: &[u32]) {
    let mut i = 0; // indice per left
    let mut j = 0; // indice per right
    let mut k = 0; // indice per result

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Copia gli elementi rimanenti
    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}
