/// Very low overhead and adaptive, with good performance when nearly sorted
/// - Stable
/// - O(n^1) worst case
/// - O(n) when nearly sorted
/// - O(0) extra space
pub fn insertion_sort(data: &mut [u32]) {
    // Start at the left so that on each pass through the left is guaranteed sorted.

    for i in 1..data.len() {
        let value = data[i];
        let mut j = i - 1;

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
}
