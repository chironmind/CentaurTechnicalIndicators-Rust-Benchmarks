use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn valleys_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Valleys", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_valleys();
            black_box(result);
        })
    });
}

criterion_group!(benches, valleys_benchmark);
criterion_main!(benches);
