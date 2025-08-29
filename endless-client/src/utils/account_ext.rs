use crate::error::EdsErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::move_types::account_address::AccountAddress;
use std::str::FromStr;

pub trait ToAccountAddress {
    fn to_account_address(&self) -> AppResult<AccountAddress>;
}

impl ToAccountAddress for &str {
    fn to_account_address(&self) -> AppResult<AccountAddress> {
        AccountAddress::from_str(self).map_err(map_err!(&EdsErr::AcctAddrParseErr))
    }
}

impl ToAccountAddress for String {
    fn to_account_address(&self) -> AppResult<AccountAddress> {
        self.as_str().to_account_address()
    }
}

impl ToAccountAddress for &[u8] {
    fn to_account_address(&self) -> AppResult<AccountAddress> {
        AccountAddress::try_from(*self).map_err(map_err!(&EdsErr::InvalidAddrLen))
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
        strs.iter().map(|addr| addr.to_account_address()).collect()
    }
}
