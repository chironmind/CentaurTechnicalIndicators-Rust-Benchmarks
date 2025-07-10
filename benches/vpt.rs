use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn vpt_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Volume Price Trend", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_vpt();
            black_box(result);
        })
    });
}

criterion_group!(benches, vpt_benchmark);
criterion_main!(benches);
