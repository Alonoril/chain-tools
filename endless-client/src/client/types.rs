use crate::utils::bcs_ext::BcsExt;
use base_infra::result::AppResult;
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::rest_client::Response;
use endless_sdk::rest_client::endless_api_types::IndexResponse;
use serde::{Deserialize, Serialize};

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
