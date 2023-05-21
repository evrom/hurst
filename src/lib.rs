//! This crate provides functionality to compute the hurst exponent using rescaled range analysis

#![deny(unused_crate_dependencies, missing_docs, unused_imports)]

use linreg::linear_regression;

mod errors;
pub mod utils;

pub use errors::*;
use utils::*;

/// Simple R/S (Rescaled range) Hurst estimation
pub fn rs_simple(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let x_mean = mean(x);
    let mean_adjusted = Vec::from_iter(x.iter().map(|x| x - x_mean));
    let cumulative_deviate = cumsum(&mean_adjusted);
    let (min, max) = minmax(&cumulative_deviate);
    let rs: f64 = (max - min) / standard_deviation(&mean_adjusted);
    rs.log2() / n.log2()
}

/// Corrected R over S Hurst exponent
pub fn rs_corrected(x: &[f64]) -> Result<f64> {
    let mut cap_x: Vec<f64> = vec![x.len() as f64];
    let mut cap_y: Vec<f64> = vec![rscalc(x)];
    let mut n: Vec<u64> = vec![0, x.len() as u64 / 2, x.len() as u64];
    // compute averaged R/S for halved intervals
    while n[1] >= 8 {
        let mut xl = Vec::with_capacity(n.len());
        let mut yl = Vec::with_capacity(n.len());
        for i in 1..n.len() {
            let rs: f64 = rscalc(&x[((n[i - 1] + 1) as usize)..(n[i] as usize)]);
            xl.push((n[i] - n[i - 1]) as f64);
            yl.push(rs);
        }
        cap_x.push(mean(&xl));
        cap_y.push(mean(&yl));
        // next step
        n = half(&n, x.len() as u64);
    }
    // apply linear regression
    let cap_x_log = Vec::from_iter(cap_x.iter().map(|a| a.ln()));
    let cap_y_log = Vec::from_iter(cap_y.iter().map(|a| a.ln()));
    let (slope, _) =
        linear_regression(&cap_x_log, &cap_y_log).map_err(|_| Error::LinearRegression)?;

    Ok(slope)
}
