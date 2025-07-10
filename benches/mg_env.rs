use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mg_env_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI McGinley Envelopes (10, 3.0)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_env();
            black_box(result);
        })
    });
}

criterion_group!(benches, mg_env_benchmark);
criterion_main!(benches);
