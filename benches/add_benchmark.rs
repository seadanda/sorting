use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sorting_rs::add;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add two numbers", |b| {
        b.iter(|| add(black_box(20), black_box(4)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
