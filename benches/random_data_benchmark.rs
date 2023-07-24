use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sorting::insertion_sort;

fn random_data_benchmark(c: &mut Criterion) {
    let test_data: Vec<u32> = black_box(vec![2, 5, 1, 6, 3, 4]);

    c.bench_function("insertion sort", |b| {
        b.iter(|| insertion_sort(test_data.clone()))
    });
}

criterion_group!(benches, random_data_benchmark);
criterion_main!(benches);
