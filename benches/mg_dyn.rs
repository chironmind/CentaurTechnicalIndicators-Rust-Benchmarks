use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mg_dyn_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI McGinley Dynamic (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_dyn();
            black_box(result);
        })
    });
}

criterion_group!(benches, mg_dyn_benchmark);
criterion_main!(benches);
