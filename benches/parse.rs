use clinical_document_parser::parser::{problem_section, problem_section_from_reader};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::fs;

const FILES: [(&str, &str); 6] = [
    ("normal_100kb", "sample-documents/generated/normal_0.1mb.xml"),
    ("normal_1mb", "sample-documents/generated/normal_1mb.xml"),
    ("medium_5mb", "sample-documents/generated/medium_5mb.xml"),
    ("medium_10mb", "sample-documents/generated/medium_10mb.xml"),
    ("big_25mb", "sample-documents/generated/big_25mb.xml"),
    ("big_50mb", "sample-documents/generated/big_50mb.xml"),
];

fn parse_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    for (name, path) in FILES {
        let bytes = fs::read(path).expect("run the generator first");
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_function(format!("{name}/path"), |b| {
            b.iter(|| problem_section(path).unwrap())
        });
        group.bench_function(format!("{name}/memory"), |b| {
            b.iter(|| problem_section_from_reader(bytes.as_slice()).unwrap())
        });
    }
    group.finish();
}

criterion_group!(benches, parse_sizes);
criterion_main!(benches);
