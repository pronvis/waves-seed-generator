use criterion::{Criterion, criterion_group, criterion_main, Benchmark};
use std::collections::HashMap;

use waves_address_generator::seed_generator::generate_seed_with_address_for_worlds;

fn criterion_benchmark(c: &mut Criterion) {
    let mut worlds_a = HashMap::new();
    worlds_a.insert(String::from("a"), 1);
    let benchmark = Benchmark::new(
        "generate_seed_with_address_for_worlds",
        move |b| b.iter(
            || generate_seed_with_address_for_worlds(&worlds_a)));
    c.bench("generate seed ending in a",
            benchmark.sample_size(5)
    );

    let mut worlds_aa = HashMap::new();
    worlds_aa.insert(String::from("aa"), 2);
    let benchmark = Benchmark::new(
        "generate_seed_with_address_for_worlds",
        move |b| b.iter(
            || generate_seed_with_address_for_worlds(&worlds_aa)));
    c.bench("generate seed ending in aa",
            benchmark.sample_size(5)
    );

    let mut worlds_aaa = HashMap::new();
    worlds_aaa.insert(String::from("aaa"), 3);
    let benchmark = Benchmark::new(
        "generate_seed_with_address_for_worlds",
        move |b| b.iter(
            || generate_seed_with_address_for_worlds(&worlds_aaa)));
    c.bench("generate seed ending in aaa",
            benchmark.sample_size(5)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);