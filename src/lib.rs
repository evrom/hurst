use stats::{stddev};

pub mod utils;

use utils::*;

/// Simple R/S Hurst estimation
pub fn rssimple(x: Vec<f64>) -> f64 {
    let n: f64 = x.len() as f64;
    let x_mean: f64 = mean(&x);
    let y: Vec<f64> = x.iter()
    .map(|x| x - x_mean)
    .collect();
    let s: Vec<f64> = cumsum(&y);
    let (min, max) = minmax(&s);
    let rs: f64 =  (max - min)/ stddev(x.clone().into_iter());
    return rs.log2() / n.log2();
}