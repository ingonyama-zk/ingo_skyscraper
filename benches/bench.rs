use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mont_square::*;


// Benchmark function
fn benchmark_sqrs(c: &mut Criterion) {
    let input: [u64; 4] = [
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
    ];
    c.bench_function("sqr_cios_ord_unr", |b| b.iter(|| sqr_cios_ord_unr(black_box(input))));
    c.bench_function("sqr_cios_opt_unr_2", |b| b.iter(|| sqr_cios_opt_unr_2(black_box(input))));
    c.bench_function("sqr_cios_opt_unr_1", |b| b.iter(|| sqr_cios_opt_unr_1(black_box(input))));
}

criterion_group!(benches, benchmark_sqrs);
criterion_main!(benches);
