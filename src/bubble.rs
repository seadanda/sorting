pub fn bubble_sort(data: &mut [u32]) {
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
