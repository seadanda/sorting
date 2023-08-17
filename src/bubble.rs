/// Sorts the elements of a mutable slice using the bubble sort algorithm.
///
/// # Arguments
///
/// * `data` - A mutable slice of elements that implement the `PartialOrd` trait.
///
/// # Examples
///
/// ```
/// use sorting::bubble_sort;
///
/// let mut data = [3, 1, 2];
/// bubble_sort(&mut data);
/// assert_eq!(data, [1, 2, 3]);
/// ```
pub fn bubble_sort<T: PartialOrd>(data: &mut [T]) {
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
}
