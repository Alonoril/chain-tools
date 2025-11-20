pub mod big_decimal;
pub mod bignum;

use crate::error::TypErr;
use alloy_primitives::U128;
use base_infra::map_err;
use base_infra::result::AppResult;

pub trait ToRsU128 {
    fn to_u128(&self) -> u128;
}
impl ToRsU128 for U128 {
    fn to_u128(&self) -> u128 {
        let bytes = self.to_le_bytes::<16>(); // 16 = 128 / 8
        u128::from_le_bytes(bytes)
    }
}
pub trait TryToRsU128 {
    fn try_to_u128(&self) -> AppResult<u128>;
}

impl TryToRsU128 for &str {
    fn try_to_u128(&self) -> AppResult<u128> {
        let val: U128 = self
            .parse::<U128>()
            .map_err(map_err!(&TypErr::ParseU128Err))?;
        Ok(val.to_u128())
    }
}

impl TryToRsU128 for String {
    fn try_to_u128(&self) -> AppResult<u128> {
        self.as_str().try_to_u128()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u128() -> AppResult<()> {
        let val = "123456789012345678901234567890";
        let rs = val.try_to_u128()?;
        assert_eq!(rs, 123456789012345678901234567890);
        Ok(())
    }
}
