use criterion::{Criterion, criterion_group, criterion_main};
use lobby::{part_1, part_1_kye, part_2, part_2_kye};

const INPUT: &str = include_str!("../input.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.benchmark_group("Part 1")
        .bench_function("ari", |b| {
            b.iter(|| part_1(INPUT));
        })
        .bench_function("kye", |b| {
            b.iter(|| part_1_kye(INPUT));
        });

    c.benchmark_group("Part 2")
        .bench_function("ari", |b| {
            b.iter(|| part_2(INPUT));
        })
        .bench_function("kye", |b| {
            b.iter(|| part_2_kye(INPUT));
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
