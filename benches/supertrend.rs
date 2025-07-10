use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn supertrend_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Supertrend (5, 2.0, Simple Ma)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_supertrend();
            black_box(result);
        })
    });
}

criterion_group!(benches, supertrend_benchmark);
criterion_main!(benches);
