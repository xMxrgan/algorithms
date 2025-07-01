/*

pub fn merge_sort(arr: &mut [u32]) {
    // Calcola la lunghezza dell'array
    // Se l'array ha un solo elemento o è vuoto, è già ordinato

    // Calcola l'indice centrale per dividere l'array in due metà

    // Crea due sottovettori copiando gli elementi della prima e della seconda metà
    // Prima metà
    // Seconda metà

    // Stampa le due metà per scopi di debug

    // Ordina ricorsivamente la prima metà
    // Ordina ricorsivamente la seconda metà

    // Unisce le due metà ordinate nell'array originale
}

fn merge(result: &mut [u32], left: &[u32], right: &[u32]) {
    // Inizializza gli indici per scorrere i tre array
    // Indice per il sottovettore sinistro (left)
    // Indice per il sottovettore destro (right)
    // Indice per l'array risultante (result)

    // Confronta gli elementi di left e right finché entrambi hanno elementi
    // Se l'elemento corrente di left è minore o uguale a quello di right,
    // copialo nell'array risultante
    // Passa all'elemento successivo in left
    // Altrimenti, copia l'elemento corrente di right nell'array risultante
    // Passa all'elemento successivo in right
    // Passa alla posizione successiva in result

    // Copia gli elementi rimanenti di left (se ce ne sono) nell'array risultante

    // Passa all'elemento successivo in left
    // Passa alla posizione successiva in result

    // Copia gli elementi rimanenti di right (se ce ne sono) nell'array risultante

    // Passa all'elemento successivo in right
    // Passa alla posizione successiva in result
}

*/

pub fn merge_sort(array: &mut [u32]) {
    let length = array.iter().len();
}
