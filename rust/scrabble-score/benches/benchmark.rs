use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scrabble_score::{score, score2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("score", |b| b.iter(|| score("OxyphenButazone")));
    c.bench_function("score2", |b| b.iter(|| score2("OxyphenButazone")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
