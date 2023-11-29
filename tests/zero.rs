use signum_sign::sgn;

#[test]
fn zero() {
    let result = sgn(0);
    assert_eq!(result, 0);
}
