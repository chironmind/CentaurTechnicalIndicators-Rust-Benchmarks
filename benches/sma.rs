use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn sma_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Smoothed Moving Average (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_sma();
            black_box(result);
        })
    });
}

criterion_group!(benches, sma_benchmark);
criterion_main!(benches);
