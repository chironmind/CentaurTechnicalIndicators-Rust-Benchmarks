use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn cci_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI CCI (14, Simple MA, STD DEV)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci();
            black_box(result);
        })
    });
}

criterion_group!(benches, cci_benchmark);
criterion_main!(benches);
