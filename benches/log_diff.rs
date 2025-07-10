use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn log_diff_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Log Difference", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_log_diff();
            black_box(result);
        })
    });
}

criterion_group!(benches, log_diff_benchmark);
criterion_main!(benches);
