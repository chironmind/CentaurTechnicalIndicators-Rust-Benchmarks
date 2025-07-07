use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn slow_so_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Slow SO (14, Simple MA)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_slow_so();
            black_box(result);
        })
    });
}

criterion_group!(benches, slow_so_benchmark);
criterion_main!(benches);
