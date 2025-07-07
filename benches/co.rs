use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn co_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI CO (7, 14, Simple MA)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_co();
            black_box(result);
        })
    });
}

criterion_group!(benches, co_benchmark);
criterion_main!(benches);
