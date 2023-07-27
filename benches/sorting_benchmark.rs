use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sorting::{bubble_sort, insertion_sort, selection_sort, shell_sort};

fn random_benchmark(c: &mut Criterion) {
    let test_data: Vec<u32> = black_box(vec![2, 5, 1, 6, 3, 4]);

    c.bench_function("insertion sort", |b| {
        b.iter(|| insertion_sort(test_data.clone()))
    });

    c.bench_function("selection sort", |b| {
        b.iter(|| selection_sort(test_data.clone()))
    });

    c.bench_function("bubble sort", |b| b.iter(|| bubble_sort(test_data.clone())));

    c.bench_function("shell sort", |b| b.iter(|| shell_sort(test_data.clone())));
}

fn sorted_benchmark(c: &mut Criterion) {
    let test_data: Vec<u32> = black_box(vec![1, 2, 3, 4, 5, 6]);

    c.bench_function("insertion sort", |b| {
        b.iter(|| insertion_sort(test_data.clone()))
    });

    c.bench_function("selection sort", |b| {
        b.iter(|| selection_sort(test_data.clone()))
    });

    c.bench_function("bubble sort", |b| b.iter(|| bubble_sort(test_data.clone())));

    c.bench_function("shell sort", |b| b.iter(|| shell_sort(test_data.clone())));
}

fn reversed_benchmark(c: &mut Criterion) {
    let test_data: Vec<u32> = black_box(vec![6, 5, 4, 3, 2, 1]);

    c.bench_function("insertion sort", |b| {
        b.iter(|| insertion_sort(test_data.clone()))
    });

    c.bench_function("selection sort", |b| {
        b.iter(|| selection_sort(test_data.clone()))
    });

    c.bench_function("bubble sort", |b| b.iter(|| bubble_sort(test_data.clone())));

    c.bench_function("shell sort", |b| b.iter(|| shell_sort(test_data.clone())));
}

fn few_unique_benchmark(c: &mut Criterion) {
    let test_data: Vec<u32> = black_box(vec![6, 5, 4, 3, 2, 1]);

    c.bench_function("insertion sort", |b| {
        b.iter(|| insertion_sort(test_data.clone()))
    });

    c.bench_function("selection sort", |b| {
        b.iter(|| selection_sort(test_data.clone()))
    });

    c.bench_function("bubble sort", |b| b.iter(|| bubble_sort(test_data.clone())));

    c.bench_function("shell sort", |b| b.iter(|| shell_sort(test_data.clone())));
}

criterion_group!(
    benches,
    random_benchmark,
    sorted_benchmark,
    reversed_benchmark,
    few_unique_benchmark
);
criterion_main!(benches);
