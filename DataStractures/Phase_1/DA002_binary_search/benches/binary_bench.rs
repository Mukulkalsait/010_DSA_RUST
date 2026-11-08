use std::hint::black_box;

use DA002_binary_search::binary::{
    binary_search,
    my_version::{binary_at_once_option, binary_at_once_res},
};
use criterion::{Criterion, criterion_group, criterion_main};

fn benchmarsing(c: &mut Criterion) {
    let size = 1_000_000;
    let data: Vec<usize> = (0..size).collect();
    let target = 999_999;
    let mut group = c.benchmark_group("Binary Search Comparision");

    group.bench_function("Orignal 3-fn split", |b| b.iter(|| binary_search(black_box(&target), black_box(&data))));
    group.bench_function("My Option version", |b| b.iter(|| binary_at_once_option(black_box(&target), black_box(&data))));
    group.bench_function("My Res Ver", |b| b.iter(|| binary_at_once_res(black_box(&target), black_box(&data))));
}

criterion_group!(benches, benchmarsing);
criterion_main!(benches);
