use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn median_abs_dev_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Median Absolute Deviation", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_median_abs_dev();
            black_box(result);
        })
    });
}

criterion_group!(benches, median_abs_dev_benchmark);
criterion_main!(benches);
