use hurst;
mod brown72;

#[test]
fn test_rssimple() {
    assert_eq!(hurst::rssimple(brown72::BROWN72.to_vec()), 0.6591636065931852f64);
}

#[test]
fn test_rs_corrected() {
    assert_eq!(hurst::rs_corrected(brown72::BROWN72.to_vec()), 0.73f64);
}

#[test]
fn test_rscalc() {
    assert_eq!(hurst::utils::rscalc(&brown72::BROWN72.to_vec()), 96.44510191365018f64);
}

#[test]
fn test_half() {
    assert_eq!(hurst::utils::half(&vec![0, 512, 1024], 1024), vec![0, 256, 512, 768, 1024]);
}

