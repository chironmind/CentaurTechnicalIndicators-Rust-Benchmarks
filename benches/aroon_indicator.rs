use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn aroon_indicator_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Aroon Indicator (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_aroon_indicator();
            black_box(result);
        })
    });
}

criterion_group!(benches, aroon_indicator_benchmark);
criterion_main!(benches);
