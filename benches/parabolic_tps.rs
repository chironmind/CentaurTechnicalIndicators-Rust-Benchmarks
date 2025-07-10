use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn parabolic_tps_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Parabolic Time Price System ", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_parabolic_tps();
            black_box(result);
        })
    });
}

criterion_group!(benches, parabolic_tps_benchmark);
criterion_main!(benches);
