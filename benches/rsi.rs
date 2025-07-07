use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn rsi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI RSI (14, Smoothed MA)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi();
            black_box(result);
        })
    });
}

criterion_group!(benches, rsi_benchmark);
criterion_main!(benches);
