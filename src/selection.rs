/// Sorts the elements of the given slice using the selection sort algorithm.
///
/// # Arguments
///
/// * `data`: A mutable reference to the slice to be sorted.
///
/// # Examples
///
/// ```
/// use sorting::selection_sort;
///
/// let mut data = [3, 1, 2];
/// selection_sort(&mut data);
/// assert_eq!(data, [1, 2, 3]);
/// ```
pub fn selection_sort<T: PartialOrd>(data: &mut [T]) {
    // scroll through the vec from i..n, pick the smallest and swap with the ith element
    // values are sorted to the left of the i index as it increases
    for i in 0..data.len() {
        let mut k = i;

        for (j, value) in data[i..].iter().enumerate() {
            if *value < data[k] {
                // this is the new minimum value, replace index and value
                k = i + j;
            }
        }

        // unstable sort, so a swap is done after every pass through the vec
        data.swap(i, k);
    }
}
