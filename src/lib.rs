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

/// Very low overhead and adaptive, with good performance when nearly sorted
/// - Stable
/// - O(n^2) worst case
/// - O(n) when nearly sorted
/// - O(1) extra space
pub fn insertion_sort(data: Vec<u32>) -> Vec<u32> {
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_simple_random_data() {
        // create a vec of sorted data and a matching shuffled vec
        let sorted_data = vec![1, 2, 3, 4, 5, 6];
        let shuffled_data = vec![2, 5, 1, 6, 3, 4];

        // load the algos
        let algos = vec![insertion_sort];

        // iterate over the algos and test them all
        for sort in algos {
            assert_eq!(sorted_data, sort(shuffled_data.clone()))
        }
    }

    #[test]
    fn sorts_simple_sorted_data() {
        // create a vec of sorted data
        let sorted_data = vec![1, 2, 3, 4, 5, 6];

        // load the algos
        let algos = vec![insertion_sort];

        // iterate over the algos and test them all
        for sort in algos {
            assert_eq!(sorted_data, sort(sorted_data.clone()))
        }
    }
}
