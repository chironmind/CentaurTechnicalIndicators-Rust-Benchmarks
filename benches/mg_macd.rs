use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mg_macd_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI McGinley MACD (7, 14)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_macd();
            black_box(result);
        })
    });
}

criterion_group!(benches, mg_macd_benchmark);
criterion_main!(benches);
