use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn cnst_env_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Constant Envelopers (10, 3.0, Smoothed MA)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cnst_env();
            black_box(result);
        })
    });
}

criterion_group!(benches, cnst_env_benchmark);
criterion_main!(benches);
