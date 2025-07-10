use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn cnst_bands_benchmark(c: &mut Criterion) {
    c.bench_function(
        "RustTI Constant bands (5, 2.0, Smoothed MA, STD Dev)",
        |b| {
            b.iter(|| {
                let result = rustti_benchmarks::compute_cnst_bands();
                black_box(result);
            })
        },
    );
}

criterion_group!(benches, cnst_bands_benchmark);
criterion_main!(benches);
