use criterion::{Criterion, criterion_group, criterion_main};
use mini_http::request::parse_request;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_request", |b| {
        b.iter(|| parse_request(black_box(&vec!["GET /index.html HTTP/1.1".to_string()])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
