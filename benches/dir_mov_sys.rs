use criterion::{criterion_group, criterion_main, Criterion};
use rustti_benchmarks;
use std::hint::black_box;

fn dir_mov_sys_benchmark(c: &mut Criterion) {
    c.bench_function("RustTI Directional Movement System (5, Simple Ma)", |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_dir_mov_sys();
            black_box(result);
        })
    });
}

criterion_group!(benches, dir_mov_sys_benchmark);
criterion_main!(benches);
