use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn donchian_chnl_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Donchian Channels (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_donchian_chnl();
            black_box(result);
        })
    });
}

criterion_group!(benches, donchian_chnl_benchmark);
criterion_main!(benches);
