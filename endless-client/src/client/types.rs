use endless_sdk::rest_client::Response;
use endless_sdk::rest_client::endless_api_types::IndexResponse;
use serde::{Deserialize, Serialize};

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
