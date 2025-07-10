use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn break_down_trends_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Break Down Trends", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_break_down_trends();
            black_box(result);
        })
    });
}

criterion_group!(benches, break_down_trends_benchmark);
criterion_main!(benches);
