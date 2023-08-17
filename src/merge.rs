/// Sorts the elements of a mutable slice using the merge sort algorithm.
///
/// # Arguments
///
/// * `data` - A mutable slice of elements that implement the `PartialOrd` and `Clone` traits.
///
/// # Examples
///
/// ```
/// use sorting::merge_sort;
///
/// let mut data = [3, 1, 2];
/// merge_sort(&mut data);
/// assert_eq!(data, [1, 2, 3]);
/// ```
pub fn merge_sort<T: PartialOrd + Clone>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let mid = data.len() / 2;

    merge_sort(&mut data[..mid]);
    merge_sort(&mut data[mid..]);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    let left = &data[..mid].to_vec();
    let right = &data[mid..].to_vec();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            data[k] = left[i].clone();
            i += 1;
        } else {
            data[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        data[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        data[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
