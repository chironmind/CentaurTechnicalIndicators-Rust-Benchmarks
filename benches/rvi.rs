use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn rvi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Relative Vigor Index", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rvi();
            black_box(result);
        })
    });
}

criterion_group!(benches, rvi_benchmark);
criterion_main!(benches);
