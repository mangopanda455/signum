use signum_sign::heron;

#[test]
fn twothousandheron() {
    let result = heron(2000f64);
    assert_eq!(result, f64::sqrt(2000f64));
}
