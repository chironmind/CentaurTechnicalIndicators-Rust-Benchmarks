use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn roi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Return on Investment", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_roi();
            black_box(result);
        })
    });
}

criterion_group!(benches, roi_benchmark);
criterion_main!(benches);
