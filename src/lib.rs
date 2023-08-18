//! A sorting crate not built for popular consumption.
//! This is just a practice crate for building and optimising common sorting algorithms.
//!
//! Here's a nice and concise list of best case scenario sorting taken from
//! [toptal](https://www.toptal.com/developers/sorting-algorithms):
//! - Stable: Equal keys aren’t reordered.
//! - Operates in place, requiring O(1) extra space.
//! - Worst-case O(n·lg(n)) key comparisons.
//! - Worst-case O(n) swaps.
//! - Adaptive: Speeds up to O(n) when data is nearly sorted or when there are few unique keys.
//!
//! In the docs the descriptions focus on highlighting main features instead of repeating the algo name.

mod bubble;
mod heap;
mod insertion;
mod merge;
mod quick;
mod selection;
mod shell;

pub use bubble::bubble_sort;
pub use heap::heap_sort;
pub use insertion::insertion_sort;
pub use merge::merge_sort;
pub use quick::quick_sort;
pub use selection::selection_sort;
pub use shell::shell_sort;

#[cfg(test)]
mod tests {
    use super::*;

    // function which runs each algo for a given input
    fn run_algos(unsorted: Vec<u32>, sorted: Vec<u32>) {
        let algos = [
            insertion_sort,
            selection_sort,
            bubble_sort,
            shell_sort,
            merge_sort,
            heap_sort,
            quick_sort,
        ];

        // iterate over the algos and test them all with the given data
        for sort in algos {
            let mut data = unsorted.clone();
            sort(&mut data);
            assert_eq!(sorted, data)
        }
    }

    #[test]
    fn they_sort_simple_random_data() {
        // create a vec of sorted data and a matching shuffled vec
        let sorted_data = vec![1, 2, 3, 4, 5, 6];
        let shuffled_data = vec![2, 5, 1, 6, 3, 4];

        // run over this data
        run_algos(shuffled_data, sorted_data);
    }

    #[test]
    fn they_sort_simple_sorted_data() {
        // create a vec of sorted data
        let sorted_data = vec![1, 2, 3, 4, 5, 6];

        // run over this data
        run_algos(sorted_data.clone(), sorted_data);
    }

    #[test]
    fn they_sort_simple_reversed_data() {
        // create a vec of sorted data and a matching reversed vec
        let sorted_data = vec![1, 2, 3, 4, 5, 6];
        let reversed_data = vec![6, 5, 4, 3, 2, 1];

        // run over this data
        run_algos(reversed_data, sorted_data);
    }

    #[test]
    fn they_sort_simple_few_unique_data() {
        // create a vec of sorted data and a matching reversed vec
        let sorted_data = vec![1, 1, 2, 2, 3, 3];
        let reversed_data = vec![2, 3, 1, 3, 1, 2];

        // run over this data
        run_algos(reversed_data, sorted_data);
    }
}
