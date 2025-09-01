use crate::error::EdsErr;
use crate::sdk_ext::types::{ChainIdCache, EntryFnArgs, ViewFnArgs};
use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::rest_client::endless_api_types::{IndexResponse, UserTransaction};
use endless_sdk::rest_client::{Client, EndlessResult, PendingTransaction, Response};
use endless_sdk::transaction_builder::TransactionBuilder;
use endless_sdk::types::chain_id::ChainId;
use endless_sdk::types::transaction::TransactionPayload;
use serde::de::DeserializeOwned;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct RestClient<'a> {
    client: &'a Client,
}

impl<'a> RestClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
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
    pub async fn simulate_fun(
        &self,
        args: EntryFnArgs<'a>,
    ) -> AppResult<Response<Vec<UserTransaction>>> {
        let chain_id = self.get_chain_id().await?;
        let overrides = args.overrides.unwrap_or_default();

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(map_err!(&EdsErr::SystemTimeErr))?
            .as_secs()
            + overrides.timeout_secs;

        let payload = TransactionPayload::EntryFunction(args.entry_fn);
        let txn_builder = TransactionBuilder::new(payload, expires_at, chain_id)
            .sender(args.signer.address())
            .sequence_number(args.signer.sequence_number())
            .max_gas_amount(overrides.max_gas_amount)
            .gas_unit_price(overrides.gas_unit_price);

        let signed_txn = args.signer.sign_with_transaction_builder(txn_builder);
        let res = self
            .client
            .simulate_with_gas_estimation(&signed_txn, true, false)
            .await
            .map_err(map_err!(&EdsErr::SimulateTxnErr))?;

        // decrement sequence number
        args.signer.decrement_sequence_number();

        Ok(res)
    }

    pub async fn entry_fun(
        &self,
        args: EntryFnArgs<'a>,
    ) -> AppResult<Response<PendingTransaction>> {
        let fn_name = args.fn_name;
        let chain_id = self.get_chain_id().await?;
        let overrides = args.overrides.unwrap_or_default();

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(map_err!(&EdsErr::SystemTimeErr))?
            .as_secs()
            + overrides.timeout_secs;

        let payload = TransactionPayload::EntryFunction(args.entry_fn);
        let txn_builder = TransactionBuilder::new(payload, expires_at, chain_id)
            .sender(args.signer.address())
            .max_gas_amount(overrides.max_gas_amount)
            .gas_unit_price(overrides.gas_unit_price);

        let signed_txn = args.signer.sign_with_transaction_builder(txn_builder);
        self.client.submit(&signed_txn).await.map_err(map_err!(
            &EdsErr::SubmitTxnErr,
            format!("function[{fn_name}]")
        ))
    }

    pub async fn view_fun<T: DeserializeOwned>(
        &self,
        args: ViewFnArgs,
    ) -> EndlessResult<Response<T>> {
        self.client.view_bcs(&args.view_fn, None).await
    }

    async fn get_index(&self) -> AppResult<Response<IndexResponse>> {
        self.client
            .get_index()
            .await
            .map_err(map_err!(&EdsErr::GetIndexErr))
    }
}
