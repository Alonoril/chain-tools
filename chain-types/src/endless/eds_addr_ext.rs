use crate::error::CtyErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use move_core_types::account_address::AccountAddress;
use std::str::FromStr;

pub trait ToEdsAddr {
    fn to_eds_addr(&self) -> AppResult<AccountAddress>;
}

impl ToEdsAddr for &str {
    fn to_eds_addr(&self) -> AppResult<AccountAddress> {
        AccountAddress::from_str(self).map_err(map_err!(&CtyErr::EdsAddrParseErr))
    }
}

impl ToEdsAddr for String {
    fn to_eds_addr(&self) -> AppResult<AccountAddress> {
        self.as_str().to_eds_addr()
    }
}

impl ToEdsAddr for &[u8] {
    fn to_eds_addr(&self) -> AppResult<AccountAddress> {
        AccountAddress::try_from(*self).map_err(map_err!(&CtyErr::InvalidAddrLen))
    }
}

pub trait ToVecBs58Str {
    fn to_bs58_strs(&self) -> Vec<String>;
}

impl ToVecBs58Str for Vec<AccountAddress> {
    fn to_bs58_strs(&self) -> Vec<String> {
        self.iter().map(|addr| addr.to_bs58_string()).collect()
    }
}

pub trait TryFromVecBs58 {
    fn from_bs58_strs(strs: &[String]) -> AppResult<Vec<AccountAddress>>;
}

impl TryFromVecBs58 for Vec<String> {
    fn from_bs58_strs(strs: &[String]) -> AppResult<Vec<AccountAddress>> {
        strs.iter().map(|addr| addr.to_eds_addr()).collect()
    }
}
