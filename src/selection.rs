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
