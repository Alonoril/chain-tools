use crate::error::EdsErr;
use crate::utils::bcs_ext::BcsExt;
use base_infra::map_err;
use base_infra::result::{AppError, AppResult};
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::rest_client::Response;
use endless_sdk::rest_client::endless_api_types::{HashValue, IndexResponse};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug)]
pub struct BaseTxnInfo<T> {
    pub timestamp: u64,
    pub success: bool,
    pub txn_hash: String,
    pub event: T,
}

impl<T> BaseTxnInfo<T> {
    pub fn new(timestamp: u64, success: bool, txn_hash: String, event: T) -> Self {
        Self {
            timestamp,
            success,
            txn_hash,
            event,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Owner(AccountAddress);
impl Owner {
    pub fn new(addr: &AccountAddress) -> Self {
        Self(*addr)
    }

    pub fn to_bytes(&self) -> AppResult<Vec<u8>> {
        self.0.to_bytes()
    }
}

impl From<Owner> for AccountAddress {
    fn from(owner: Owner) -> Self {
        owner.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token(AccountAddress);
impl Token {
    pub fn new(addr: &AccountAddress) -> Self {
        Self(*addr)
    }

    pub fn to_bytes(&self) -> AppResult<Vec<u8>> {
        self.0.to_bytes()
    }
}

impl From<Token> for AccountAddress {
    fn from(token: Token) -> Self {
        token.0
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IndexData {
    pub epoch: u64,
    pub ledger_version: u64,
    pub oldest_ledger_version: u64,
    pub ledger_timestamp: u64,
    pub oldest_block_height: u64,
    pub block_height: u64,
}

impl From<Response<IndexResponse>> for IndexData {
    fn from(resp: Response<IndexResponse>) -> Self {
        let (idx, _state) = resp.into_parts();
        Self {
            epoch: idx.epoch.into(),
            ledger_version: idx.ledger_version.0,
            oldest_ledger_version: idx.oldest_ledger_version.0,
            ledger_timestamp: idx.ledger_timestamp.0,
            oldest_block_height: idx.oldest_block_height.0,
            block_height: idx.block_height.0,
        }
    }
}

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub enum TxHashVal {
//     HashVal(HashValue),
//     HashStr(String),
// }

/// A transaction hash wrapper that normalizes multiple caller inputs.
pub struct TxHashVal(HashValue);

/// Converts a caller-provided transaction hash into `HashValue`.
pub trait TryIntoTxHashValue {
    fn try_into_hash_value(self) -> AppResult<HashValue>;
}

impl From<TxHashVal> for HashValue {
    fn from(val: TxHashVal) -> Self {
        val.0
    }
}

impl From<HashValue> for TxHashVal {
    fn from(hash: HashValue) -> Self {
        Self(hash)
    }
}

impl TryIntoTxHashValue for HashValue {
    fn try_into_hash_value(self) -> AppResult<HashValue> {
        Ok(self)
    }
}

impl TryIntoTxHashValue for &HashValue {
    fn try_into_hash_value(self) -> AppResult<HashValue> {
        Ok(*self)
    }
}

impl TryIntoTxHashValue for TxHashVal {
    fn try_into_hash_value(self) -> AppResult<HashValue> {
        Ok(self.into())
    }
}

impl TryIntoTxHashValue for &str {
    fn try_into_hash_value(self) -> AppResult<HashValue> {
        TxHashVal::try_from(self).map(Into::into)
    }
}

impl TryIntoTxHashValue for String {
    fn try_into_hash_value(self) -> AppResult<HashValue> {
        TxHashVal::try_from(self).map(Into::into)
    }
}

impl TryIntoTxHashValue for &String {
    fn try_into_hash_value(self) -> AppResult<HashValue> {
        TxHashVal::try_from(self.as_str()).map(Into::into)
    }
}

impl Deref for TxHashVal {
    type Target = HashValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for TxHashVal {
    type Error = AppError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let hash = s
            .parse::<HashValue>()
            .map_err(map_err!(&EdsErr::ParseHashValue, s))?;
        Ok(Self(hash))
    }
}

impl TryFrom<String> for TxHashVal {
    type Error = AppError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}
