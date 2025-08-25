pub mod types;

use crate::client::types::{IndexData, ViewResponse};
use crate::error::EdsErr;
use crate::sdk::rest_client::RestClient;
use crate::sdk::types::{EntryFnArgs, ViewFnArgs};
use crate::utils::bcs_ext::BcsExt;
use base_infra::map_err;
use base_infra::result::{AppResult, DynErrCode};
use endless_sdk::helper_client::Overrides;
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::rest_client::endless_api_types::UserTransaction;
use endless_sdk::rest_client::{Client, PendingTransaction, Response, Transaction};
use endless_sdk::types::LocalAccount;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::FromStr;
use tracing::info;
use url::Url;

#[derive(Clone)]
pub struct BaseClient {
    pub client: Client,
}

impl BaseClient {
    pub fn new(node_url: &str) -> AppResult<Self> {
        let node_url = Url::from_str(node_url).map_err(map_err!(&EdsErr::InvalidNodeUrl))?;
        Ok(Self {
            client: Client::new(node_url),
        })
    }

    pub fn rest_client(&self) -> RestClient<'_> {
        RestClient::new(&self.client)
    }

    pub async fn get_index(&self) -> AppResult<IndexData> {
        let res = self
            .client
            .get_index()
            .await
            .map_err(map_err!(&EdsErr::GetVersionErr))?;
        Ok(res.into())
    }

    pub async fn simulate_transfer(
        &self,
        from_account: &LocalAccount,
        to_account: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Vec<UserTransaction>>> {
        let args = vec![to_account.to_bytes()?, amount.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer", from_account);

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, vec![])?
            .with_overrides(overrides);

        self.rest_client().simulate_fun(fn_args).await
    }

    pub async fn transfer(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>> {
        let args = vec![to.to_bytes()?, amount.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer", from);

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, vec![])?
            .with_overrides(overrides);

        self.rest_client().entry_fun(fn_args).await
    }
    pub async fn simulate_transfer_token(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        token: AccountAddress,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Vec<UserTransaction>>> {
        let args = vec![to.to_bytes()?, amount.to_bytes()?, token.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer_coins", from);
        let t_args = vec!["0x1::fungible_asset::Metadata"];

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, t_args)?
            .with_overrides(overrides);

        self.rest_client().simulate_fun(fn_args).await
    }

    pub async fn transfer_coins(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        token: AccountAddress,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>> {
        let args = vec![to.to_bytes()?, amount.to_bytes()?, token.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer_coins", from);
        let t_args = vec!["0x1::fungible_asset::Metadata"];

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, t_args)?
            .with_overrides(overrides);

        self.rest_client().entry_fun(fn_args).await
    }

    pub async fn get_token_balance(
        &self,
        owner: AccountAddress,
        token: AccountAddress,
    ) -> AppResult<ViewResponse<u128>> {
        let args = vec![owner.to_bytes()?, token.to_bytes()?];
        let t_args = vec!["0x1::fungible_asset::Metadata"];
        let (mun, fun) = ("primary_fungible_store", "balance");

        let args = ViewFnArgs::new(AccountAddress::ONE, mun, fun, args, t_args)?;
        let res = self
            .view_fn_with_state(args, &EdsErr::GetTokenBalance, None)
            .await?;
        Ok(res.into())
    }

    pub async fn get_eds_balance(&self, owner: &AccountAddress) -> AppResult<ViewResponse<u128>> {
        let (args, t_args) = (vec![owner.to_bytes()?], vec![]);
        let args = ViewFnArgs::new(AccountAddress::ONE, "endless_coin", "balance", args, t_args)?;
        let res = self
            .view_fn_with_state(args, &EdsErr::GetEdsBalance, None)
            .await?;
        Ok(res.into())
    }

    pub async fn view_fn_with_state<T: DeserializeOwned + Debug>(
        &self,
        args: ViewFnArgs,
        code: &'static DynErrCode,
        ext_msg: Option<String>,
    ) -> AppResult<ViewResponse<T>> {
        let resp = self.rest_client().view_fun(args).await;
        let resp = if let Some(msg) = ext_msg {
            resp.map_err(map_err!(code, msg))?
        } else {
            resp.map_err(map_err!(code))?
        };

        let state = resp.state().clone();
        let (_, lps): (u8, T) = resp.into_inner();
        Ok(ViewResponse::new(state, lps))
    }

    pub async fn entry_fn_with_gas_txn(
        &self,
        args: EntryFnArgs<'_>,
        gas_used: Option<u64>,
        code: &'static DynErrCode,
    ) -> AppResult<ViewResponse<Transaction>> {
        let fn_name = args.fn_name;
        let mut overrides = None;
        if let Some(gas_used) = gas_used {
            let max_gas_amount = gas_used + 100;
            info!("do entry_fn[{fn_name}] with max_gas_amount: {max_gas_amount}");
            overrides = Some(Overrides {
                max_gas_amount,
                ..Overrides::default()
            });
        }

        let fn_args = args.with_overrides(overrides);
        let pending_tx = self
            .rest_client()
            .entry_fun(fn_args)
            .await
            .map_err(map_err!(code, format!("function[{fn_name}]")))?
            .into_inner();

        info!("entry_fn[{fn_name}] pending_tx: {}", pending_tx.hash);
        self.wait_for_txn(&pending_tx).await
    }

    pub async fn wait_for_txn(
        &self,
        pending_tx: &PendingTransaction,
    ) -> AppResult<ViewResponse<Transaction>> {
        let resp = self
            .client
            .wait_for_transaction(pending_tx)
            .await
            .map_err(map_err!(&EdsErr::WaitForTxnErr))?;

        let state = resp.state().clone();
        Ok(ViewResponse::new(state, resp.into_inner()))
    }
}
