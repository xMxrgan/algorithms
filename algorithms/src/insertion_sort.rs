pub fn insertion_sort(array: &mut [u32]) {
    let length = array.len();

    for i in 1..=length - 1 {
        let mut j = i;

        println!("{:?}", array);
        println!("{}", array[j]);

        while j > 0 && array[j - 1] > array[j] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}
