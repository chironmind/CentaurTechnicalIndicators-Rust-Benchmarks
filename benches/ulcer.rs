use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn ulcer_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Ulcer Index (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ulcer();
            black_box(result);
        })
    });
}

criterion_group!(benches, ulcer_benchmark);
criterion_main!(benches);
