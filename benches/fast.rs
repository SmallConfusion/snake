use boards::{get_board_2, get_board_4};
use criterion::{Criterion, criterion_group, criterion_main};
use snake::engine::evaluate;
use std::hint::black_box;

mod boards;

pub fn bench(c: &mut Criterion) {
    c.bench_function("4 depth 1", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_4()),
                black_box(1),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    c.bench_function("4 depth 2", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_4()),
                black_box(2),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    c.bench_function("2 depth 2", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_4()),
                black_box(2),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    c.bench_function("2 depth 3", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_2()),
                black_box(3),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    c.bench_function("2 depth 4", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_2()),
                black_box(4),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    c.bench_function("4 depth 3", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_4()),
                black_box(3),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });
}

criterion_group!(benches_fast, bench);
criterion_main!(benches_fast);
