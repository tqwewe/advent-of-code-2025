use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use lobby::{
    parse_battery_banks, part_1, part_1_kye, part_2, part_2_bank, part_2_kye, part_2_kye_bank,
};

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
            b.iter(|| {
                let banks: Vec<&str> = INPUT.trim().split('\n').collect();
                part_2(&banks)
            });
        })
        .bench_function("ari single-bank", |b| {
            let banks: Vec<&str> = INPUT.trim().split('\n').collect();
            b.iter_custom(|iters| {
                let start = Instant::now();
                for i in 0..iters {
                    let bank = &banks[(i % banks.len() as u64) as usize];
                    std::hint::black_box(part_2_bank(bank));
                }
                start.elapsed()
            });
        })
        .bench_function("kye", |b| {
            b.iter(|| {
                let input = parse_battery_banks(INPUT);
                part_2_kye(&input)
            });
        })
        .bench_function("kye no-parsing", |b| {
            let input = parse_battery_banks(INPUT);
            b.iter(|| part_2_kye(&input));
        })
        .bench_function("kye single-bank", |b| {
            let banks = parse_battery_banks(INPUT);
            b.iter_custom(|iters| {
                let start = Instant::now();
                for i in 0..iters {
                    let bank = &banks[(i % banks.len() as u64) as usize];
                    std::hint::black_box(part_2_kye_bank(bank));
                }
                start.elapsed()
            });
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
