use hurst;
mod brown72;

#[test]
fn test_rssimple() {
    assert_eq!(hurst::rs_simple(&brown72::BROWN72), 0.6591636065931848f64);
}

#[test]
fn test_rs_corrected() {
    assert_eq!(
        hurst::rs_corrected(&brown72::BROWN72).unwrap(),
        0.7394639422875734f64
    );
}

#[test]
fn test_rscalc() {
    assert_eq!(
        hurst::utils::rscalc(&brown72::BROWN72.to_vec()),
        96.44510191365002f64
    );
}

#[test]
fn test_half() {
    assert_eq!(
        hurst::utils::half(&vec![0, 512, 1024], 1024),
        vec![0, 256, 512, 768, 1024]
    );
}

#[test]
fn test_standard_deviation() {
    assert_eq!(
        hurst::utils::standard_deviation(&brown72::BROWN72.to_vec()),
        2.951991553652619f64
    );
}
