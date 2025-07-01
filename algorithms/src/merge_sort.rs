pub fn merge_sort(arr: &mut [u32]) {
    // Calcola la lunghezza dell'array
    let len = arr.len();
    // Se l'array ha un solo elemento o è vuoto, è già ordinato
    if len <= 1 {
        return;
    }

    // Calcola l'indice centrale per dividere l'array in due metà
    let mid = len / 2;

    // Crea due sottovettori copiando gli elementi della prima e della seconda metà
    let mut left = arr[..mid].to_vec(); // Prima metà
    let mut right = arr[mid..].to_vec(); // Seconda metà

    // Stampa le due metà per scopi di debug
    println!("{:?} – {:?}", left, right);

    // Ordina ricorsivamente la prima metà
    merge_sort(&mut left);
    // Ordina ricorsivamente la seconda metà
    merge_sort(&mut right);

    // Unisce le due metà ordinate nell'array originale
    merge(arr, &left[..], &right[..]);
}

fn merge(result: &mut [u32], left: &[u32], right: &[u32]) {
    // Inizializza gli indici per scorrere i tre array
    let mut i = 0; // Indice per il sottovettore sinistro (left)
    let mut j = 0; // Indice per il sottovettore destro (right)
    let mut k = 0; // Indice per l'array risultante (result)

    // Confronta gli elementi di left e right finché entrambi hanno elementi
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            // Se l'elemento corrente di left è minore o uguale a quello di right,
            // copialo nell'array risultante
            result[k] = left[i];
            i += 1; // Passa all'elemento successivo in left
        } else {
            // Altrimenti, copia l'elemento corrente di right nell'array risultante
            result[k] = right[j];
            j += 1; // Passa all'elemento successivo in right
        }
        k += 1; // Passa alla posizione successiva in result
    }

    // Copia gli elementi rimanenti di left (se ce ne sono) nell'array risultante
    while i < left.len() {
        result[k] = left[i];
        i += 1; // Passa all'elemento successivo in left
        k += 1; // Passa alla posizione successiva in result
    }

    // Copia gli elementi rimanenti di right (se ce ne sono) nell'array risultante
    while j < right.len() {
        result[k] = right[j];
        j += 1; // Passa all'elemento successivo in right
        k += 1; // Passa alla posizione successiva in result
    }
}
