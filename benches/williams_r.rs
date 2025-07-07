use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn williams_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Williams %R (14)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_williams_r();
            black_box(result);
        })
    });
}

criterion_group!(benches, williams_benchmark);
criterion_main!(benches);
