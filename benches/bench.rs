use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ingo_skyscrapper::*;


fn benchmark(c: &mut Criterion) {
    let x: [u64; 4] = [rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()];
    let y: [u64; 4] = [rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()];
    // let x_u256 = unsafe { std::mem::transmute::<[u64; 4], U256>(x) };
    // let y_u256 = unsafe { std::mem::transmute::<[u64; 4], U256>(y) };
    // c.bench_function("sqr_cios_ord_unr", |b| b.iter(|| sqr_cios_ord_unr(black_box(input))));
    // c.bench_function("sqr_cios_opt", |b| b.iter(|| sqr_cios_opt(black_box(x))));
    // c.bench_function("sqr_cios_opt_unr_1", |b| b.iter(|| sqr_cios_opt_unr_1(black_box(x))));
    // c.bench_function("sqr_cios_opt_unr_2", |b| b.iter(|| sqr_cios_opt_unr_2(black_box(x))));
    // c.bench_function("sqr_cios_opt_unr_2_tweaked", |b| b.iter(|| sqr_cios_opt_unr_2_tweaked(black_box(x))));
    // c.bench_function("mul_cios_opt_unr_1", |b| b.iter(|| mul_cios_opt_unr_1(black_box(input), black_box(input2))));
    // c.bench_function("bar_u8", |b| b.iter(|| bar_u8(black_box(x))));
    // c.bench_function("arith_sub", |b| b.iter(|| overflowing_sub(black_box(x), black_box(y))));
    // c.bench_function("arith_sub_u256", |b| b.iter(|| black_box(x_u256).overflowing_sub(black_box(y_u256))));
    c.bench_function("compress", |b| b.iter(|| compress(black_box(x), black_box(y))));
    c.bench_function("compress", |b| b.iter(|| compress(black_box(x), black_box(y))));
    c.bench_function("compress", |b| b.iter(|| compress(black_box(x), black_box(y))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
