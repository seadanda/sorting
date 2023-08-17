pub fn quick_sort<T: PartialOrd>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    // divide and conquer => recursive

    // select pivot and order data
    let pivot = partition(data);

    // recurse into both sides
    quick_sort(&mut data[..pivot]);
    quick_sort(&mut data[pivot + 1..]);
}

fn partition<T: PartialOrd>(data: &mut [T]) -> usize {
    let pivot = data.len() - 1;
    let mut i = 0;

    // check if less than pivot and swap
    for j in 0..pivot {
        if data[j] < data[pivot] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, pivot);
    i
}
