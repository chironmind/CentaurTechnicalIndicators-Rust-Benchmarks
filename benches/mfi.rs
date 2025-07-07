use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn mfi_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI MFI (14)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mfi();
            black_box(result);
        })
    });
}

criterion_group!(benches, mfi_benchmark);
criterion_main!(benches);
