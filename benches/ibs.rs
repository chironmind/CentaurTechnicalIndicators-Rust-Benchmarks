use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn ibs_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Internal Bar Strength", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ibs();
            black_box(result);
        })
    });
}

criterion_group!(benches, ibs_benchmark);
criterion_main!(benches);
