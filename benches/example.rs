#[macro_use]
extern crate criterion;
extern crate whatlang;
extern crate serde_json;

use criterion::{Criterion, ParameterizedBenchmark, Throughput, black_box};
use std::collections::HashMap;
use whatlang::{detect, detect_script};

const EXAMPLE_DATA: &'static str = include_str!("../tests/examples.json");
const EXAMPLE_SIZE: usize = 8 * 1024;

fn extend_string(s: &mut String) {
    let data = s.clone();
    let mut chars = data.chars().cycle();
    while s.len() < EXAMPLE_SIZE {
        s.push(chars.next().unwrap());
    }
}

fn bench_detect(c: &mut Criterion) {
    let mut examples: HashMap<String, String> = serde_json::from_str(EXAMPLE_DATA).unwrap();
    for text in examples.values_mut() {
        extend_string(text);
    }

    c.bench_function("detect_language", move |b| {
        b.iter(|| {
            for (_, text) in &examples {
                black_box(|| detect(text));
            }
        })
    });
}

fn bench_detect_script(c: &mut Criterion) {
    let mut examples: HashMap<String, String> = serde_json::from_str(EXAMPLE_DATA).unwrap();
    for text in examples.values_mut() {
        extend_string(text);
    }

    c.bench_function("detect_script", move |b| {
        b.iter(|| {
            for (_, text) in &examples {
                black_box(|| detect_script(text));
            }
        })
    });
}

criterion_group!(benches, bench_detect_script, bench_detect);
criterion_main!(benches);
