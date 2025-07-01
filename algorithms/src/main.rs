mod heap_sort;
mod insertion_sort;
mod merge_sort;

use rand::Rng;
use std::time::Instant;

fn main() {
    let mut random = rand::thread_rng();

    let array: Vec<u32> = (0..10).map(|_| random.gen_range(1..=100)).collect();

    let real_time = Instant::now();
    println!("Insertion sort started at: {:?}", real_time);
    insertion_sort::insertion_sort(&mut array.clone());
    let duration = real_time.elapsed();
    println!("And lasted: {:?}", duration.as_secs());

    let real_time = Instant::now();
    println!("Merge sort started at: {:?}", real_time);
    merge_sort::merge_sort(&mut array.clone());
    let duration = real_time.elapsed();
    println!("And lasted: {:?} milliseconds", duration.as_secs());
}
