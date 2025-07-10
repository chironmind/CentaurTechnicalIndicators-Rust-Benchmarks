use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn stdev_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Standard Deviation", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_stdev();
            black_box(result);
        })
    });
}

criterion_group!(benches, stdev_benchmark);
criterion_main!(benches);
