pub fn sgn(x: i128) -> i128 {
    if x > 0 {
        return 1;
    } else if x < 0 {
        return -1;
    } else if x == 0 {
        return 0;
    } else {
        return -256;
    }
}
