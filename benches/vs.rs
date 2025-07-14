use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn vs_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Volatility System (5, Simple MA)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_vs();
            black_box(result);
        })
    });
}

criterion_group!(benches, vs_benchmark);
criterion_main!(benches);
