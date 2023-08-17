pub fn selection_sort(data: &mut [u32]) {
    // scroll through the vec from i..n, pick the smallest and swap with the ith element
    // values are sorted to the left of the i index as it increases
    let mut k;
    let mut k_value;
    for i in 0..data.len() {
        k = i;
        k_value = data[k];

        for (j, &value) in data[i..].iter().enumerate() {
            if value < k_value {
                // this is the new minimum value, replace index and value
                k = i + j;
                k_value = value;
            }
        }

        // unstable sort, so a swap is done after every pass through the vec
        data.swap(i, k);
    }
}
