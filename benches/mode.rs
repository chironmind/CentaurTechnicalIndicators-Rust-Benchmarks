use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mode_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Mode", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mode();
            black_box(result);
        })
    });
}

criterion_group!(benches, mode_benchmark);
criterion_main!(benches);
