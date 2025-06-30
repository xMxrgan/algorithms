mod insertion_sort;
mod merge_sort;

use rand::Rng;

fn main() {
    let mut random = rand::thread_rng();
    let array: Vec<u32> = (0..=10000).map(|_| random.gen_range(1..=100)).collect();

    println!("Insertion");
    insertion_sort::insertion_sort(&mut array.clone());

    println!("Merge");
    merge_sort::merge_sort(&mut array.clone());
}
