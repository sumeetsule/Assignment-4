#[allow(dead_code)]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[allow(dead_code)]
pub fn sub(a: f64, b: f64) -> f64 {
    a - b
}

#[allow(dead_code)]
pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

#[allow(dead_code)]
pub fn div(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Second operand cannot be 0 for division");
    }
    Ok(a / b)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(-2.0, -3.0), -5.0);
        assert_eq!(add(2.0, -3.0), -1.0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5.0, 3.0), 2.0);
        assert_eq!(sub(-5.0, -3.0), -2.0);
        assert_eq!(sub(5.0, -3.0), 8.0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2.0, 3.0), 6.0);
        assert_eq!(mul(-2.0, -3.0), 6.0);
        assert_eq!(mul(2.0, -3.0), -6.0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(6.0, 3.0), Ok(2.0));
        assert_eq!(div(-6.0, -3.0), Ok(2.0));
        assert_eq!(div(6.0, -3.0), Ok(-2.0));
        assert_eq!(div(1.0, 0.0), Err("Second operand cannot be 0 for division"));
    }

}