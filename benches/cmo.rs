use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn cmo_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI CMO (14)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cmo();
            black_box(result);
        })
    });
}

criterion_group!(benches, cmo_benchmark);
criterion_main!(benches);
