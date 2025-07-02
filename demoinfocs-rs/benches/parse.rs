use criterion::{Criterion, criterion_group, criterion_main};
use demoinfocs_rs::parser::Parser;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;

fn bench_parse_to_end(c: &mut Criterion) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../test/cs-demos/default.dem");
    let mut data = Vec::new();
    File::open(&path).unwrap().read_to_end(&mut data).unwrap();
    c.bench_function("parse_to_end_default", |b| {
        b.iter(|| {
            let mut parser = Parser::new(Cursor::new(&data));
            parser.parse_to_end().unwrap();
        });
    });
}

criterion_group!(benches, bench_parse_to_end);
criterion_main!(benches);
