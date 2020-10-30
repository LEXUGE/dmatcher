use criterion::{criterion_group, criterion_main, Criterion};
use dmatcher::Dmatcher;
use std::fs::File;
use std::io::Read;

fn bench_match(c: &mut Criterion) {
    let mut file = File::open("./benches/accelerated-domains.china.raw.txt").unwrap();
    let mut contents = String::new();
    let mut matcher = Dmatcher::new();
    file.read_to_string(&mut contents).unwrap();
    matcher.insert_lines(&contents);
    c.bench_function("match", |b| {
        b.iter(|| matcher.matches("你好.store.www.baidu.com"))
    });
}

criterion_group!(benches, bench_match);
criterion_main!(benches);
