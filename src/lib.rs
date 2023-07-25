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
pub fn insertion_sort(mut data: Vec<u32>) -> Vec<u32> {
    // Start at the left so that on each pass through the left is guaranteed sorted.
    let mut j;

    for i in 1..data.len() {
        let value = data[i];
        j = i - 1;

        // keep shifting left while it's lower than the element to its left
        loop {
            if value < data[j] {
                data.swap(j + 1, j);
            }

            // when we hit leftmost element, advance to the next untested element
            if j == 0 {
                break;
            }

            j -= 1;
        }
    }
    data
}

pub fn selection_sort(mut data: Vec<u32>) -> Vec<u32> {
    // scroll through the vec from i..n, pick the smallest and swap with the ith element
    // values are sorted to the left of the i index as it increases
    let mut k;
    let mut k_value;
    for i in 0..data.len() {
        k = i;
        // k_value minimises the number of times we call data.get()
        k_value = data[k];

        for j in i..data.len() {
            if data[j] < k_value {
                // this is the new minimum value, replace index and value
                k = j;
                k_value = data[j];
            }
        }

        // unstable sort, so a swap is done after every pass through the vec
        data.swap(i, k);
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Get the name of the function to identify which algo failed in stdout for failing tests.
    fn get_algo_name<T>(_: T) -> &'static str {
        // return the type, which includes the function name
        std::any::type_name::<T>()
    }

    #[test]
    fn sorts_simple_random_data() {
        // create a vec of sorted data and a matching shuffled vec
        let sorted_data = vec![1, 2, 3, 4, 5, 6];
        let shuffled_data = vec![2, 5, 1, 6, 3, 4];

        // load the algos
        let algos = vec![insertion_sort, selection_sort];

        // iterate over the algos and test them all
        for sort in algos {
            println!("{}", get_algo_name(sort));
            assert_eq!(sorted_data, sort(shuffled_data.clone()))
        }
    }

    #[test]
    fn sorts_simple_sorted_data() {
        // create a vec of sorted data
        let sorted_data = vec![1, 2, 3, 4, 5, 6];

        // load the algos
        let algos = vec![insertion_sort, selection_sort];

        // iterate over the algos and test them all
        for sort in algos {
            println!("{}", get_algo_name(sort));
            assert_eq!(sorted_data, sort(sorted_data.clone()))
        }
    }
}
