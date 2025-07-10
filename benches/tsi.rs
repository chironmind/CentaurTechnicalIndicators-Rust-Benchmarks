use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn tsi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI True Strength Index (5, 10, Simple Ma)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_tsi();
            black_box(result);
        })
    });
}

criterion_group!(benches, tsi_benchmark);
criterion_main!(benches);
