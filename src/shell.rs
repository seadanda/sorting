pub fn shell_sort(data: &mut [u32]) {
    // Compare and rearrange elements like insertion sort but with comparison across intervals > 1
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
}
