use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn pi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Positivity Indicator", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_pi();
            black_box(result);
        })
    });
}

criterion_group!(benches, pi_benchmark);
criterion_main!(benches);
