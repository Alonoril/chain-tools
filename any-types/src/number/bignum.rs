use bigdecimal::BigDecimal;

///
/// Extension trait for BigDecimal to provide power operations.
///
pub trait NumPowExt<T> {
    ///
    /// Raises a BigDecimal to the power of `exp` using `base` as the base for exponentiation.
    ///
    /// # Arguments
    ///
    /// * `base` - The base number to be raised to a power.
    /// * `exp` - The exponent (can be negative).
    ///
    /// # Returns
    ///
    /// * `AppResult<BigDecimal>` - The result of raising the base to the given exponent.
    ///
    fn mul_powi(&self, base: u64, exp: i32) -> T;
}

impl NumPowExt<BigDecimal> for BigDecimal {
    fn mul_powi(&self, base: u64, exp: i32) -> BigDecimal {
        if exp == 0 {
            return self * BigDecimal::from(1);
        }

        let val = BigDecimal::from(base.pow(exp.abs() as u32));
        if exp.is_negative() {
            return self / val;
        }
        self * val
    }
}

impl NumPowExt<f64> for f64 {
    fn mul_powi(&self, base: u64, exp: i32) -> f64 {
        let base = base as f64;
        base.powi(exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powi_10() {
        let a = BigDecimal::from(2);
        assert_eq!(a.mul_powi(10, 3), BigDecimal::from(2000));
    }

    #[test]
    fn test_neg_powi_10() {
        let a = BigDecimal::from(2);
        assert_eq!(a.mul_powi(10, -3), BigDecimal::new(2.into(), 3i64));
        assert_eq!(a.mul_powi(10, -3).to_string(), "0.002".to_string());
    }
}
