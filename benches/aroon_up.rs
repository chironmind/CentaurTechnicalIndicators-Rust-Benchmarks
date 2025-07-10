use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn aroon_up_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Aroon up (5)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_aroon_up();
            black_box(result);
        })
    });
}

criterion_group!(benches, aroon_up_benchmark);
criterion_main!(benches);
