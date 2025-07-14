use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn corr_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Correlation (5, Simple MA, Std Dev)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_corr();
            black_box(result);
        })
    });
}

criterion_group!(benches, corr_benchmark);
criterion_main!(benches);
