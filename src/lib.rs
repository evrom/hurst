use wasm_bindgen::prelude::*;
use stats::{mean, stddev};

#[wasm_bindgen]
pub fn rssimple(data: Vec<f64>) -> f64 {
    let y: Vec<f64> = data.iter()
    .map(|x| x - mean(data.clone().into_iter()))
    .collect();
    let s: Vec<f64> = y.iter()
    .scan(0f64, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    }).collect();
    let (min, max) = s
    .iter()
    .fold((s[0], s[0]), |acc, &x| (acc.0.min(x), acc.1.max(x)));
println!("{}, {}", min, max);
    let rs: f64 =  (min - max)/ stddev(data.clone().into_iter());

    return rs;
}
