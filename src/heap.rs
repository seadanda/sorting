pub fn heap_sort(data: &mut [u32]) {
    // create max heap
    for i in (0..data.len() / 2).rev() {
        heapify(data, data.len(), i);
    }

    // extract elements from heap
    for i in (1..data.len()).rev() {
        data.swap(0, i);
        heapify(data, i, 0);
    }
}

fn heapify(data: &mut [u32], len: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    // grab both children and compare
    if left < len && data[left] > data[largest] {
        largest = left;
    }

    if right < len && data[right] > data[largest] {
        largest = right;
    }

    // ensure that root is always max
    if largest != root {
        data.swap(root, largest);
        heapify(data, len, largest);
    }
}
