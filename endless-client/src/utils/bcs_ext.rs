use crate::error::EdsErr;
use base_infra::result::{AppResult, any_err};

pub trait BcsExt {
    fn to_bytes(&self) -> AppResult<Vec<u8>>
    where
        Self: serde::Serialize,
    {
        bcs::to_bytes(self).map_err(any_err(&EdsErr::ToBcsBytes))
    }
}
impl<T: serde::Serialize> BcsExt for T {}
