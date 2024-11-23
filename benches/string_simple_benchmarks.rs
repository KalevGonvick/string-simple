use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use string_simple::compare;
use string_simple::builder::StringBuilder;

fn compare_benchmark(c: &mut Criterion) {
    c.bench_function("compare", |b| b.iter(|| compare::contains(
        black_box(&String::from("this is my test string for benchmarks!")),
        black_box(&String::from("test"))
    )));
}

fn builder_benchmark(c: &mut Criterion) {
    c.bench_function("builder", |b| b.iter(|| {
        let mut test = StringBuilder::new();
        test.append(black_box("this")).append(black_box("is")).append(black_box("a")).append(black_box("test!")).build();
    }));
}

criterion_group!(benches, compare_benchmark, builder_benchmark);
criterion_main!(benches);