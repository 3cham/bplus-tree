use rand::random;
use std::collections::BTreeMap;
use bplus_tree::BTree;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_get(c: &mut Criterion) {
    let mut t = BTree::new();
    t.insert(1, 1);
    c.bench_function("BTree::get", |b| b.iter(|| t.get(1)));
}

fn bench_insert(c: &mut Criterion) {
    let mut t = BTree::<u32>::new();
    let mut st = BTreeMap::<u32, u32>::new();
    c.bench_function("BTree::insert", |b| b.iter(|| t.insert(random(), random())));
}

fn bench_insert_vs_std_collection(c: &mut Criterion) {
    let mut t = BTree::<u32>::new();
    let mut st = BTreeMap::<u32, u32>::new();
    let mut group = c.benchmark_group("Benchmark vs std collections");
    group.bench_function("ours::insert", |b| b.iter(|| t.insert(random(), random())));
    group.bench_function("std::insert", |b| b.iter(|| st.insert(random(), random())));
    group.finish();
}

criterion_group!(benches, bench_insert_vs_std_collection);
criterion_main!(benches);
