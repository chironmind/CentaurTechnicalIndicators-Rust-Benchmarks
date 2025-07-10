use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn log_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Log", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_log();
            black_box(result);
        })
    });
}

criterion_group!(benches, log_benchmark);
criterion_main!(benches);
