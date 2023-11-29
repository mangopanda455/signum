use signum_sign::sgn;

#[test]
fn positive() {
    let result = sgn(2);
    assert_eq!(result, 1);
}
