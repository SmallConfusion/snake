use boards::{get_board_2, get_board_4};
use criterion::{Criterion, criterion_group, criterion_main};
use snake::engine::evaluate;
use std::hint::black_box;

mod boards;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Medium");
    group.sample_size(10);

    group.bench_function("4 depth 4", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_4()),
                black_box(4),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    group.bench_function("2 depth 5", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_2()),
                black_box(5),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    group.bench_function("2 depth 6", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_2()),
                black_box(6),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    group.bench_function("2 depth 7", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_2()),
                black_box(7),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });

    group.bench_function("2 depth 8", |b| {
        b.iter(|| {
            evaluate(
                black_box(&get_board_2()),
                black_box(8),
                black_box(std::time::Instant::now() + std::time::Duration::from_secs(9999999)),
            )
        })
    });
}

criterion_group!(benches_medium, bench);
criterion_main!(benches_medium);
