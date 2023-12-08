#![cfg(test)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sokoban_level_generator::generate_level;

fn level_generation(c: &mut Criterion) {
    let variants = vec![
        (1, 1, 1),
        (2, 1, 1),
        (1, 2, 1),
        (2, 2, 1),
        (3, 2, 1),
        (2, 3, 1),
        (3, 3, 1),
        (4, 3, 1),
        (3, 4, 1),
        (4, 4, 1),
        (2, 1, 2),
        (1, 2, 2),
        (2, 2, 2),
        (3, 2, 2),
        (3, 3, 2),
        (4, 3, 2),
        (4, 4, 2),
        (2, 3, 3),
        (3, 3, 3),
        (3, 4, 3),
        (4, 4, 3),
    ];

    let mut group = c.benchmark_group("generate level");

    for (width, height, box_count) in variants {
        group.bench_with_input(
            format!("w: {}, h: {}, boxes: {}", width, height, box_count),
            &(width, height, box_count),
            |b, (width, height, box_count)| b.iter(|| generate_level(black_box(*width), black_box(*height), black_box(*box_count))),
        );
    }

    group.finish();
}

criterion_group!(benches, level_generation);
criterion_main!(benches);
