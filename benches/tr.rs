use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn tr_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI True Range", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_tr();
            black_box(result);
        })
    });
}

criterion_group!(benches, tr_benchmark);
criterion_main!(benches);
