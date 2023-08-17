pub fn shell_sort<T: PartialOrd>(data: &mut [T]) {
    // Compare and rearrange elements like insertion sort but with comparison across intervals > 1
    // n/2, n/4, ... 1 decreasing intervals
    let mut interval = data.len() / 2;

    while interval > 0 {
        for i in interval..data.len() {
            let mut j = i;

            while j >= interval && data[j - interval] > data[j] {
                data.swap(j, j - interval);
                j -= interval;
            }
        }
        // normal div of two integers gives us the floor
        interval /= 2;
    }
}
