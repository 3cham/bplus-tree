use bplus_tree::BTree;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_get(c: &mut Criterion) {
    let mut t = BTree::new();
    t.insert(1, 1);
    c.bench_function("BTree::get", |b| b.iter(|| t.get(1)));
}

criterion_group!(benches, bench_get);
criterion_main!(benches);
