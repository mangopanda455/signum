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

pub fn heron(s: f64) -> f64 {
    let mut r = 100_f64;
    let mut e = 0_f64;
    let mut i = 0;
    while i < 1000 {
        e = (s - (r * r)) / (2_f64 * r);
        r += e;
        i += 1;
    }
    return r;
}
