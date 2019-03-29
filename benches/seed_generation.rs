use criterion::{Criterion, criterion_group, criterion_main, Benchmark};
use std::collections::HashMap;

use waves_address_generator::seed_generator::generate_seed_with_address_for_worlds;

fn criterion_benchmark(c: &mut Criterion) {
    let mut worlds = HashMap::new();
    worlds.insert(String::from("xxx"), 3);
    worlds.insert(String::from("yyyy"), 4);
    worlds.insert(String::from("invis"), 5);
    let bench_name = format!("generate seeds for map {:?}", &worlds);
    let benchmark = Benchmark::new(
        "generate_seed_with_address_for_worlds",
        move |b| b.iter(
            || generate_seed_with_address_for_worlds(&worlds)));
    c.bench(&bench_name,
            benchmark.sample_size(10)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);