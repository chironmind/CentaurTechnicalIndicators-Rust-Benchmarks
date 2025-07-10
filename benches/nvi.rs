use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn nvi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Negative Volume Index", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_nvi();
            black_box(result);
        })
    });
}

criterion_group!(benches, nvi_benchmark);
criterion_main!(benches);
