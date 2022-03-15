use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
#[path = "../src/main.rs"]
mod main;
// test for n nodes
fn test_nodes(n: i32) {
    let mut tree: main::RedBlackTree<i32> = main::RedBlackTree::new();
    for i in 0..n {
        tree.insert_node(i);
    }
    for i in 0..n/10 {
        tree.find_node(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let sizes: Vec<i32> = vec!(10000, 40000, 70000, 100000, 130000);
    let mut group = c.benchmark_group("RB Tree Search and insert");
    for size in sizes.iter() {
        group.sample_size(100);  
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| test_nodes(size));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);