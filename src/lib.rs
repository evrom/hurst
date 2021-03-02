use linreg::linear_regression;

pub mod utils;

use utils::*;

/// Simple R/S Hurst estimation
pub fn rssimple(x: Vec<f64>) -> f64 {
    let n: f64 = x.len() as f64;
    let x_mean: f64 = mean(&x);
    let y: Vec<f64> = x.iter().map(|x| x - x_mean).collect();
    let s: Vec<f64> = cumsum(&y);
    let (min, max) = minmax(&s);
    let rs: f64 = (max - min) / standard_deviation(&x);
    return rs.log2() / n.log2();
}

/// Corrected R over S Hurst exponent
pub fn rs_corrected(x: Vec<f64>) -> f64 {
    let mut cap_x: Vec<f64> = vec![x.len() as f64];
    let mut cap_y: Vec<f64> = vec![rscalc(&x)];
    let mut n: Vec<u64> = vec![0, x.len() as u64 / 2, x.len() as u64];
    // compute averaged R/S for halved intervals
    while n[1] >= 8 {
        let mut xl: Vec<f64> = vec![];
        let mut yl: Vec<f64> = vec![];
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
    let cap_x_log: Vec<f64> = cap_x.iter().map(|a| a.ln()).collect();
    let cap_y_log: Vec<f64> = cap_y.iter().map(|a| a.ln()).collect();
    let (slope, _): (f64, f64) = linear_regression(&cap_x_log, &cap_y_log).unwrap();
    return slope;
}
