use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sorting::*;

fn random_benchmark(c: &mut Criterion) {
    let mut insertion_test_data: Vec<u32> = black_box(vec![2, 5, 1, 6, 3, 4]);

    c.bench_function("insertion sort", |b| {
        b.iter(|| insertion_sort(&mut insertion_test_data))
    });
}

criterion_group!(benches, random_benchmark);
criterion_main!(benches);
