use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mg_cci_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI McGinley CCI (14, Mean Abs Dev)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_cci();
            black_box(result);
        })
    });
}

criterion_group!(benches, mg_cci_benchmark);
criterion_main!(benches);
