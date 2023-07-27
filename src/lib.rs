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

pub fn bubble_sort(mut data: Vec<u32>) -> Vec<u32> {
    let mut swapped;
    loop {
        swapped = false;
        for i in 1..data.len() {
            if data[i - 1] > data[i] {
                data.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    data
}

pub fn shell_sort(mut data: Vec<u32>) -> Vec<u32> {
    // Compare and rearrange elements like insertion sort but with intervals > 1
    // n/2, n/4, ... 1 decreasing intervals
    let mut interval = data.len() / 2;
    let mut j;
    let mut temp;

    while interval > 0 {
        for i in interval..data.len() {
            temp = data[i];
            j = i;

            while j >= interval && data[j - interval] > temp {
                data.swap(j, j - interval);
                j -= interval;
            }
        }
        // normal div of two integers gives us the floor
        interval /= 2;
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

    // function which runs each algo for a given input
    fn run_algos(unsorted: Vec<u32>, sorted: Vec<u32>) {
        let algos = vec![insertion_sort, selection_sort, bubble_sort, shell_sort];

        // iterate over the algos and test them all with the given data
        for sort in algos {
            dbg!(get_algo_name(&sort));
            assert_eq!(sorted, sort(unsorted.clone()))
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
