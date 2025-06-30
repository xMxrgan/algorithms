mod insertion_sort;
mod merge_sort;

use rand::Rng;

fn main() {
    let mut random = rand::thread_rng();
    let mut array: Vec<u32> = (0..10).map(|_| random.gen_range(1..=100)).collect();

    insertion_sort::insertion_sort(&mut array);
}
