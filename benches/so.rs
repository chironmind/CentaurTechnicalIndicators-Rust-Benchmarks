use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn so_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI SO (14)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_so();
            black_box(result);
        })
    });
}

criterion_group!(benches, so_benchmark);
criterion_main!(benches);
