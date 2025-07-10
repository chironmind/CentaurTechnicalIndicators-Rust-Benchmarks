use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn atr_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Average True Range", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_atr();
            black_box(result);
        })
    });
}

criterion_group!(benches, atr_benchmark);
criterion_main!(benches);
