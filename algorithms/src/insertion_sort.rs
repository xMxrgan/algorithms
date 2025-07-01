pub fn insertion_sort(array: &mut [u32]) {
    let length = array.len();

    println!("ARRAY:{:?}", array);

    for i in 1..length {
        let mut k = i;

        while k > 0 && array[k - 1] > array[k] {
            array.swap(k - 1, k);
            k -= 1;
        }
        println!("{:?} – K:{}, I:{}", array, k, i);
    }
}
