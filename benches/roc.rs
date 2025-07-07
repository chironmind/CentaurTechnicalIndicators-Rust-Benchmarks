use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn roc_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI RoC (14)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_roc();
            black_box(result);
        })
    });
}

criterion_group!(benches, roc_benchmark);
criterion_main!(benches);
