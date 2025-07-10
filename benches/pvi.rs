use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn pvi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Positive Volume Index", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_pvi();
            black_box(result);
        })
    });
}

criterion_group!(benches, pvi_benchmark);
criterion_main!(benches);
