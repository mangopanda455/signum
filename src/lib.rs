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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive() {
        let result = sgn(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn negative() {
        let result = sgn(-2);
        assert_eq!(result, -1);
    }

    #[test]
    fn zero() {
        let result = sgn(0);
        assert_eq!(result, 0);
    }
}
