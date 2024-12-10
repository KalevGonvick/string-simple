use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use string_simple::{compare, modify};
use string_simple::builder::StringBuilder;

fn compare_benchmark(c: &mut Criterion) {
    c.bench_function("contains", |b| b.iter(|| compare::contains(
        black_box(&String::from("this is my test string for benchmarks!")),
        black_box(&String::from("test"))
    )));
    c.bench_function("contains SIMD", |b| b.iter(|| {
        let needles = String::from("test");
        let chunk = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let mut haystack = String::with_capacity(16384);
        for _ in 0..254 {
            haystack.push_str(chunk);
        }
        haystack.push_str("test");
        compare::contains_simd(black_box(&haystack), black_box(&needles))
    }));
    c.bench_function("sub_count_simd SIMD", |b| b.iter(|| {
        let needles = String::from("test");
        let chunk = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let mut haystack = String::with_capacity(16384);
        haystack.push_str("test");
        for _ in 0..254 {
            haystack.push_str(chunk);
        }
        haystack.push_str("test");
        compare::substring_count_simd(black_box(&haystack), black_box(&needles))
    }));
    c.bench_function("all substrings from char group", |b| b.iter(|| compare::substring_char_group_count(
        black_box(&String::from("aabbcc")),
        black_box(&vec!['a', 'b', 'c'])
    )));
    c.bench_function("char count", |b| b.iter(|| {
        let needles = vec!['a', 'b', 'c'];
        let chunk = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let mut haystack = String::with_capacity(16384);
        for _ in 0..255 {
            haystack.push_str(chunk);
        }
        compare::count_chars(black_box(&haystack), black_box(&needles))
    }
    ));
    c.bench_function("char count SIMD", |b| b.iter(|| {
        let needles = vec!['a', 'b', 'c'];
        let chunk = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let mut haystack = String::with_capacity(16384);
        for _ in 0..255 {
            haystack.push_str(chunk);
        }
        compare::count_chars_simd(black_box(&haystack), black_box(&needles))
    }));
    c.bench_function("find all", |b| b.iter(|| compare::find_all_exact(
        black_box(&String::from("aaabbbbccc")),
        black_box(&String::from("abbb"))
    )));

}

fn builder_benchmark(c: &mut Criterion) {
    c.bench_function("builder", |b| b.iter(|| {
        let mut test = StringBuilder::new();
        test.append(black_box("this")).append(black_box("is")).append(black_box("a")).append(black_box("test!")).build();
    }));
}

fn modify_benchmark(c: &mut Criterion) {
    c.bench_function("append", |b| b.iter(|| {
        let mut str1 = String::from("abc");
        modify::append(
            black_box(&mut str1),
            black_box(&String::from("abc"))
        );
    }));
    c.bench_function("replace", |b| b.iter(|| {
        let mut str1 = String::from("abctestabc");
        modify::replace(
            black_box(&mut str1),
            black_box(&String::from("test")),
            black_box(&String::from("replaced"))
        );
    }));
}

criterion_group!(benches, compare_benchmark, builder_benchmark, modify_benchmark);
criterion_main!(benches);