use std::hint::black_box;

use cafeteria::{part_1, part_2, part_2_kye};
use criterion::{Criterion, criterion_group, criterion_main};

const INPUT: &str = include_str!("../input.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.benchmark_group("Part 1").bench_function("ari", |b| {
        b.iter(|| black_box(part_1(INPUT)));
    });
    // .bench_function("kye", |b| {
    //     b.iter(|| black_box(part_1_kye(INPUT)));
    // });

    c.benchmark_group("Part 2")
        .sample_size(10_000)
        .bench_function("ari", |b| {
            b.iter(|| black_box(part_2(INPUT)));
        })
        .bench_function("kye", |b| {
            b.iter(|| black_box(part_2_kye(INPUT)));
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
