use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn ema_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Exponential Moving Average (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ema();
            black_box(result);
        })
    });
}

criterion_group!(benches, ema_benchmark);
criterion_main!(benches);
