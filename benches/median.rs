use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn median_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Median", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_median();
            black_box(result);
        })
    });
}

criterion_group!(benches, median_benchmark);
criterion_main!(benches);
