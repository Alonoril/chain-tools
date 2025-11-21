use crate::error::TypErr;
use crate::number::ToRsU128;
use alloy_primitives::{U128, U256};
use base_infra::map_err;
use base_infra::result::AppResult;
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint, Sign};
use std::str::FromStr;

pub trait ToBigDec {
    /// If the floating-point number (f32, f64)
    /// or the large string floating-point number are converted to BigDecimal,
    /// the actual accuracy of the float is subject to the actual accuracy of the floating point number,
    /// and the parameters decimals are ignored
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal>;
}

impl ToBigDec for U256 {
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal> {
        Ok(u256_to_big_decimal_scaled(self, decimals as i64))
    }
}

impl ToBigDec for U128 {
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal> {
        Ok(u128_to_big_decimal_scaled(self, decimals as i64))
    }
}

impl ToBigDec for u128 {
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal> {
        U128::from(self).to_big_dec(decimals)
    }
}

impl ToBigDec for u64 {
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal> {
        U128::from(self).to_big_dec(decimals)
    }
}

impl ToBigDec for f64 {
    fn to_big_dec(self, _dec: u8) -> AppResult<BigDecimal> {
        // let val = BigDecimal::from_f64(self).ok_or(app_err!(&TypErr::F64ToBigDecErr))?;
        BigDecimal::try_from(self).map_err(map_err!(&TypErr::F64ToBigDecErr))
    }
}

impl ToBigDec for f32 {
    fn to_big_dec(self, _dec: u8) -> AppResult<BigDecimal> {
        BigDecimal::try_from(self).map_err(map_err!(&TypErr::F32ToBigDecErr))
    }
}

impl ToBigDec for &str {
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal> {
        if self.find('.').is_some() {
            let msg = format!("`String` value[{self}]");
            return BigDecimal::from_str(self).map_err(map_err!(&TypErr::BigDecFromStr, msg));
        }

        let val = U256::from_str(self).map_err(map_err!(&TypErr::U256FromStr))?;
        val.to_big_dec(decimals)
    }
}

impl ToBigDec for &String {
    fn to_big_dec(self, decimals: u8) -> AppResult<BigDecimal> {
        let this: &str = &self;
        this.to_big_dec(decimals)
    }
}

pub trait BigDecToU128 {
    /// BigDecimal(123.456789) -> U128(123456789)
    fn to_scaled_u128(&self) -> AppResult<U128>;

    /// Convert to blockchain-supported uint type with precision conversion
    /// Example: BigDecimal(123.456789) -> u128(123456789)
    fn to_chain_u128(&self) -> AppResult<u128>;
}

impl BigDecToU128 for BigDecimal {
    fn to_scaled_u128(&self) -> AppResult<U128> {
        let decimals = self.fractional_digit_count();
        big_decimal_to_u128(&self, decimals).map_err(map_err!(&TypErr::U128ToBigDec))
    }

    // self.to_u128().ok_or_else(else_err!(&TypErr::BigDecToRsU128))
    fn to_chain_u128(&self) -> AppResult<u128> {
        self.to_scaled_u128().map(|e| e.to_u128())
    }
}

pub trait BigDecToU256 {
    fn to_u256(self, decimals: u8) -> AppResult<U256>;
}

impl BigDecToU256 for BigDecimal {
    fn to_u256(self, decimals: u8) -> AppResult<U256> {
        big_decimal_to_u256(&self, decimals as i64).map_err(map_err!(&TypErr::U256ToBigDec))
    }
}

/// Errors related to conversion
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalConvertError {
    /// BigDecimal contains a fractional part exceeding the given `decimals`
    FractionalPart,
    /// Negative numbers cannot be mapped to unsigned integers
    NegativeNumber,
    /// Value exceeds the range representable by U256/U128
    Overflow,
}

impl core::fmt::Display for DecimalConvertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecimalConvertError::FractionalPart => {
                write!(f, "BigDecimal fractional part exceeds precision")
            }
            DecimalConvertError::NegativeNumber => write!(f, "value is negative"),
            DecimalConvertError::Overflow => write!(f, "value exceeds target type maximum"),
        }
    }
}

impl std::error::Error for DecimalConvertError {}

/// 10 raised to the power of `exp`, returning a `BigInt`
fn pow10(exp: i64) -> BigInt {
    let mut result = BigInt::from(1u8);
    for _ in 0..exp {
        result *= 10u8;
    }
    result
}

/// Convert U256 to BigDecimal with dynamic `decimals` support
pub fn u256_to_big_decimal_scaled(value: U256, decimals: i64) -> BigDecimal {
    let buf: [u8; 32] = value.to_be_bytes();
    let big_uint = BigUint::from_bytes_be(&buf);
    let big_int = BigInt::from_biguint(Sign::Plus, big_uint);
    // BigDecimal::new: v * 10^(-scale)
    BigDecimal::new(big_int, decimals)
}

/// Convert U128 to BigDecimal with dynamic `decimals` support
pub fn u128_to_big_decimal_scaled(value: U128, decimals: i64) -> BigDecimal {
    let buf: [u8; 16] = value.to_be_bytes();
    let big_uint = BigUint::from_bytes_be(&buf);
    let big_int = BigInt::from_biguint(Sign::Plus, big_uint);
    BigDecimal::new(big_int, decimals)
}

/// BigDecimal -> U256，给定 `decimals`
pub fn big_decimal_to_u256(value: &BigDecimal, decimals: i64) -> Result<U256, DecimalConvertError> {
    let (big_int, scale) = value.clone().into_bigint_and_exponent();

    // BigDecimal = big_int * 10^(-scale)
    if scale > decimals {
        return Err(DecimalConvertError::FractionalPart);
    }
    if big_int.sign() == Sign::Minus {
        return Err(DecimalConvertError::NegativeNumber);
    }

    let factor = pow10(decimals - scale);
    let scaled = big_int * factor; // Should now be an integer representation
    let big_uint = scaled
        .to_biguint()
        .ok_or(DecimalConvertError::NegativeNumber)?;

    let bytes = big_uint.to_bytes_be();
    if bytes.len() > 32 {
        return Err(DecimalConvertError::Overflow);
    }
    let mut buf = [0u8; 32];
    buf[32 - bytes.len()..].copy_from_slice(&bytes);
    Ok(U256::from_be_bytes(buf))
}

/// Convert BigDecimal to U128, given `decimals`
pub fn big_decimal_to_u128(value: &BigDecimal, decimals: i64) -> Result<U128, DecimalConvertError> {
    let (big_int, scale) = value.clone().into_bigint_and_exponent();

    if scale > decimals {
        return Err(DecimalConvertError::FractionalPart);
    }
    if big_int.sign() == Sign::Minus {
        return Err(DecimalConvertError::NegativeNumber);
    }

    let factor = pow10(decimals - scale);
    let scaled = big_int * factor;
    let big_uint = scaled
        .to_biguint()
        .ok_or(DecimalConvertError::NegativeNumber)?;

    let bytes = big_uint.to_bytes_be();
    if bytes.len() > 16 {
        return Err(DecimalConvertError::Overflow);
    }
    let mut buf = [0u8; 16];
    buf[16 - bytes.len()..].copy_from_slice(&bytes);
    Ok(U128::from_be_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn to_rs_u128() {
        let val = U128::from(123456789u128);
        let dec = u128_to_big_decimal_scaled(val, 6);
        println!("{:?}", dec);
        let back = dec.to_chain_u128().unwrap();
        println!("{:?}", back);
        assert_eq!(val.to_u128(), back);
    }

    #[test]
    fn roundtrip_u256_decimals_0() {
        let val = U256::from(123456789u128);
        let dec = u256_to_big_decimal_scaled(val, 0);
        let back = big_decimal_to_u256(&dec, 0).unwrap();
        assert_eq!(val, back);
    }

    #[test]
    fn roundtrip_u256_decimals_18() {
        let original = U256::from(1_234_567_890_123_456_789u128);
        let big = u256_to_big_decimal_scaled(original, 18);
        // Should equal 0.001234567890123456789
        let expected = BigDecimal::from_str("0.001234567890123456789").unwrap();
        assert_eq!(big, expected);
        let back = big_decimal_to_u256(&big, 18).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn roundtrip_u128_decimals_6() {
        let original = U128::from(1_000_001u128);
        let big = u128_to_big_decimal_scaled(original, 6);
        let expected = BigDecimal::from_str("1.000001").unwrap();
        assert_eq!(big, expected);
        let back = big_decimal_to_u128(&big, 6).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn fractional_part_error() {
        let bd = BigDecimal::from_str("1.00001").unwrap();
        assert_eq!(
            big_decimal_to_u128(&bd, 4).unwrap_err(),
            DecimalConvertError::FractionalPart
        );
    }

    #[test]
    fn negative_error() {
        let bd = BigDecimal::from_str("-1.0").unwrap();
        assert_eq!(
            big_decimal_to_u256(&bd, 18).unwrap_err(),
            DecimalConvertError::NegativeNumber
        );
    }

    #[test]
    fn overflow_error() {
        // 2^260 > U256::MAX
        let mut big_int = BigInt::from(1u8);
        big_int = big_int << 260;
        let bd = BigDecimal::new(big_int, 0);
        assert_eq!(
            big_decimal_to_u256(&bd, 0).unwrap_err(),
            DecimalConvertError::Overflow
        );
    }

    #[test]
    fn str_big_decimal_to_big_decimal() {
        let val = "573994446227.2625155503857009566842";
        let value = val.to_big_dec(24).unwrap();
        println!("value: {value:?}");
        assert_eq!(value.to_string(), val);
    }
}
