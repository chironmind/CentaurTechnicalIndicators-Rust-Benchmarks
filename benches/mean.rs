use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mean_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Mean", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mean();
            black_box(result);
        })
    });
}

criterion_group!(benches, mean_benchmark);
criterion_main!(benches);
