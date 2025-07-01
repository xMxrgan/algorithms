#!/usr/bin/env python3

import time
import random

def insertion_sort(array):
    length = len(array)

    print(f"ARRAY: {array}")

    for i in range(1, length):
        k = i

        while k > 0 and array[k - 1] > array[k]:
            array[k - 1], array[k] = array[k], array[k - 1]
            k -= 1
        print(f"{array} – K:{k}, I:{i}")

# Esempio di utilizzo
if __name__ == "__main__":
    # Numero di elementi nell'array
    num_elements = 10000  # Modifica questo valore per cambiare la dimensione dell'array

    # Genera un array casuale con `num_elements` numeri interi tra 1 e 100
    array = [random.randint(1, 10) for _ in range(num_elements)]

    # Timer di inizio
    start_time = time.time()

    # Esegui l'algoritmo di ordinamento
    insertion_sort(array)

    # Timer di fine
    end_time = time.time()

    # Stampa il tempo di esecuzione
    print(f"Array ordinato: {array}")
    print(f"Tempo di esecuzione: {end_time - start_time:.6f} secondi")
