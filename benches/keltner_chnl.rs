use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn keltner_chnl_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Keltner Channel (5, 2.0, Exp MA, Simple Ma)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_keltner_chnl();
            black_box(result);
        })
    });
}

criterion_group!(benches, keltner_chnl_benchmark);
criterion_main!(benches);
