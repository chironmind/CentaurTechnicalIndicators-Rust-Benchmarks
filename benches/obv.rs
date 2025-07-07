use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn obv_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI OBV", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_obv();
            black_box(result);
        })
    });
}

criterion_group!(benches, obv_benchmark);
criterion_main!(benches);
