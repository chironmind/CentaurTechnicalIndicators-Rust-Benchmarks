use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mg_bands_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI McGinley bands (5, 2.0, STD Dev)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_bands();
            black_box(result);
        })
    });
}

criterion_group!(benches, mg_bands_benchmark);
criterion_main!(benches);
