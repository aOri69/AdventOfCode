#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use day_05::{part_2_single, part_2_threaded, part_2_threaded_mpsc};

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("test.txt");
    let mut group = c.benchmark_group("Day 05 / Part 2");
    // for i in [20u64, 21u64].iter() {
    group.bench_with_input(
        BenchmarkId::new("Part 2", "Single-threaded"),
        input,
        |b, i| b.iter(|| part_2_single(i)),
    );
    group.bench_with_input(
        BenchmarkId::new("Part 2", "Multi-threaded"),
        input,
        |b, i| b.iter(|| part_2_threaded(i)),
    );
    group.bench_with_input(
        BenchmarkId::new("Part 2", "Multi-threaded MPSC"),
        input,
        |b, i| b.iter(|| part_2_threaded_mpsc(i)),
    );
    // }
    group.finish();
}

criterion_group!(benches, bench_part2);
criterion_main!(benches);
