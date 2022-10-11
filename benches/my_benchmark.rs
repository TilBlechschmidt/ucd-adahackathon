use criterion::{black_box, criterion_group, criterion_main, Criterion};
use invoice_generator::*;

fn create_invoice_benchmark(c: &mut Criterion) {
    c.bench_function("create invoice [n=1000]", |b| {
        b.iter(|| build_invoice(black_box(1000)))
    });
}

fn total_benchmark(c: &mut Criterion) {
    c.bench_function("calculate total price [n=1000]", |b| {
        let invoice = build_invoice(black_box(1000));
        b.iter(|| invoice.price())
    });
}

fn stats_benchmark(c: &mut Criterion) {
    c.bench_function("calculate stats [n=1000]", |b| {
        let invoice = build_invoice(black_box(1000));
        b.iter(|| invoice.statistics())
    });
}

fn formatting_benchmark(c: &mut Criterion) {
    c.bench_function("format invoice [n=1000]", |b| {
        let invoice = build_invoice(black_box(1000));
        b.iter(|| invoice.to_string())
    });
}

fn everything_benchmark(c: &mut Criterion) {
    c.bench_function("full billing process [n=1000]", |b| {
        b.iter(|| {
            let invoice = build_invoice(black_box(1000));
            (invoice.price(), invoice.statistics(), invoice.to_string())
        })
    });
}

criterion_group!(
    parts,
    create_invoice_benchmark,
    total_benchmark,
    stats_benchmark,
    formatting_benchmark
);
criterion_group!(process, everything_benchmark);
criterion_main!(parts, process);
