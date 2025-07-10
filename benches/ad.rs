use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn ad_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Acumulation Distribution ", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ad();
            black_box(result);
        })
    });
}

criterion_group!(benches, ad_benchmark);
criterion_main!(benches);
