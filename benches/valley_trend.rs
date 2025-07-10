use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn valley_trend_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Valley Trend", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_valley_trend();
            black_box(result);
        })
    });
}

criterion_group!(benches, valley_trend_benchmark);
criterion_main!(benches);
