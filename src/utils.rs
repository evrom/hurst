//! This module provides utility functions.

/// Compute the mean of a slice
pub fn mean(x: &[f64]) -> f64 {
    let sum: f64 = x.iter().sum();
    let n: f64 = x.len() as f64;
    sum / n
}

/// Compute the standard deviation of a slice
pub fn standard_deviation(x: &[f64]) -> f64 {
    let mean_x: f64 = mean(x);
    let sum_x_minus_mean: f64 = x.iter().map(|a| (a - mean_x).powi(2)).sum();
    (sum_x_minus_mean / (x.len() as f64)).sqrt()
}

/// Compute the cumulative sum of a slice
pub fn cumsum(x: &[f64]) -> Vec<f64> {
    Vec::from_iter(x.iter().scan(0f64, |acc, &a| {
        *acc += a;
        Some(*acc)
    }))
}

/// Compute the minimum and maximum of a slice
pub fn minmax(x: &[f64]) -> (f64, f64) {
    x.iter()
        .fold((x[0], x[0]), |acc, &x| (acc.0.min(x), acc.1.max(x)))
}

/// define the R/S scale
pub fn rscalc(x: &[f64]) -> f64 {
    let x_mean: f64 = mean(x);
    let x_minus_mean: Vec<f64> = x.iter().map(|x| x - x_mean).collect();
    let y: Vec<f64> = cumsum(&x_minus_mean);
    let (min_y, max_y) = minmax(&y);
    let r: f64 = (max_y - min_y).abs();
    let s: f64 = standard_deviation(x);
    r / s
}

/// half intervals of indices
pub fn half(n: &[u64], original_length: u64) -> Vec<u64> {
    let previous_step: u64 = n[1];
    let next_step: u64 = previous_step / 2;
    let length: u64 = original_length / next_step;
    let range: Vec<u64> = (0..length + 1).collect();
    range.iter().map(|a| a * next_step).collect()
}
