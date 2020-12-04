use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let lines: Vec<String> = day3::read_lines().expect("unable to load input file");

    let forest = day3::Forest::new(lines);

    let mut traverser = day3::Traverser::new(&forest);

    c.bench_function("part one", |b| b.iter(|| traverser.part_one()));
    c.bench_function("part two", |b| b.iter(|| traverser.part_two()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
