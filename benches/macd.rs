use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn macd_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI MACD (7, 14, Simple MA)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_macd();
            black_box(result);
        })
    });
}

criterion_group!(benches, macd_benchmark);
criterion_main!(benches);
