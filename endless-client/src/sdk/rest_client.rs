use crate::error::SdkErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::rest_client::endless_api_types::IndexResponse;
use endless_sdk::rest_client::{Client, EndlessResult, PendingTransaction, Response};
use endless_sdk::transaction_builder::TransactionBuilder;
use endless_sdk::types::chain_id::ChainId;
use endless_sdk::types::transaction::TransactionPayload;
use serde::de::DeserializeOwned;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::sdk::types::{ChainIdCache, EntryFnArgs, ViewFnArgs};

#[derive(Clone, Debug)]
pub struct RestClient<'a> {
    rest: &'a Client,
}

impl<'a> RestClient<'a> {
    pub fn new(rest: &'a Client) -> Self {
        Self { rest }
    }

    pub async fn get_index(&self) -> AppResult<Response<IndexResponse>> {
        self.rest
            .get_index()
            .await
            .map_err(map_err!(&SdkErr::GetIndexErr))
    }

    pub async fn get_chain_id(&self) -> AppResult<ChainId> {
        if let Some(chain_id) = ChainIdCache.get().await {
            return Ok(chain_id);
        }

        let chain_id = self.get_index().await?.inner().chain_id;
        let chain_id = ChainId::new(chain_id);
        ChainIdCache.set(chain_id).await;
        Ok(chain_id)
    }

    pub async fn entry_fun(
        &self,
        args: EntryFnArgs<'a>,
    ) -> AppResult<Response<PendingTransaction>> {
        let chain_id = self.get_chain_id().await?;
        let overrides = args.overrides.unwrap_or_default();

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(map_err!(&SdkErr::SystemTimeErr))?
            .as_secs()
            + overrides.timeout_secs;

        let payload = TransactionPayload::EntryFunction(args.entry_fn);
        let txn_builder = TransactionBuilder::new(payload, expires_at, chain_id)
            .sender(args.signer.address())
            .max_gas_amount(overrides.max_gas_amount)
            .gas_unit_price(overrides.gas_unit_price);

        let signed_txn = args.signer.sign_with_transaction_builder(txn_builder);
        self.rest
            .submit(&signed_txn)
            .await
            .map_err(map_err!(&SdkErr::SubmitTxnErr))
    }

    pub async fn view_fun<T: DeserializeOwned>(
        &self,
        args: ViewFnArgs,
    ) -> EndlessResult<Response<T>> {
        self.rest.view_bcs(&args.view_fn, None).await
    }
}
