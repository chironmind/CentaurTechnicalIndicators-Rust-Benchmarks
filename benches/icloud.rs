use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn icloud_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Ichimoku Cloud (5, 10, 15)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_icloud();
            black_box(result);
        })
    });
}

criterion_group!(benches, icloud_benchmark);
criterion_main!(benches);
