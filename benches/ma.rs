use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn ma_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Simple Moving Average (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ma();
            black_box(result);
        })
    });
}

criterion_group!(benches, ma_benchmark);
criterion_main!(benches);
