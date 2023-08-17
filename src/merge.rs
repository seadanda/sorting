pub fn merge_sort(data: &mut [u32]) {
    if data.len() <= 1 {
        return;
    }

    let r = data.len() / 2;
    let mut sub_a: Vec<u32> = data[..r].to_vec();
    let mut sub_b: Vec<u32> = data[r..].to_vec();

    merge_sort(&mut sub_a);
    merge_sort(&mut sub_b);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < sub_a.len() && j < sub_b.len() {
        if sub_a[i] < sub_b[j] {
            data[k] = sub_a[i];
            i += 1;
        } else {
            data[k] = sub_b[j];
            j += 1;
        }
        k += 1;
    }

    while i < sub_a.len() {
        data[k] = sub_a[i];
        i += 1;
        k += 1;
    }

    while j < sub_b.len() {
        data[k] = sub_b[j];
        j += 1;
        k += 1;
    }
}
