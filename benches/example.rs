#[macro_use]
extern crate criterion;
extern crate whatlang;
extern crate serde_json;

use criterion::{Criterion, ParameterizedBenchmark, Throughput};
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

    let mut langs: Vec<String> = examples.keys().map(|lang| lang.to_string()).collect();
    langs.sort();

    c.bench_function_over_inputs("detect_language", move |b, lang| {
        let text = &examples[lang];
        b.iter(|| detect(text))
    }, langs);
}

fn bench_detect_script(c: &mut Criterion) {
    let mut examples: HashMap<String, String> = serde_json::from_str(EXAMPLE_DATA).unwrap();
    for text in examples.values_mut() {
        extend_string(text);
    }
    let examples2 = examples.clone();

    let mut langs: Vec<String> = examples.keys().map(|lang| lang.to_string()).collect();
    langs.sort();

    c.bench(
        "detect_script",
        ParameterizedBenchmark::new(
            "",
            move |b, lang| {
                let text = &examples[lang];
                b.iter(|| detect_script(text));
            },
            langs
        ).throughput(move |lang| Throughput::Bytes(examples2[lang].len() as u32))
    );
}

criterion_group!(benches, bench_detect_script, bench_detect);
criterion_main!(benches);
