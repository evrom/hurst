use hurst;
mod brown72;

#[test]
fn test_rssimple() {
    assert_eq!(hurst::rssimple(brown72::BROWN72.to_vec()), 0.6591636065931852f64);
}

#[test]
fn test_mean() {
    assert_eq!(hurst::utils::mean(&brown72::BROWN72.to_vec()), 39.881204287109334f64);
}