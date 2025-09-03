use crate::client::EnhancedClient;
use crate::client::types::{Owner, Token};
use crate::error::EdsErr;
use crate::sdk_ext::account::LocalAccountExt;
use crate::sdk_ext::types::{EntryFnArgs, ViewFnArgs};
use crate::utils::bcs_ext::BcsExt;
use base_infra::result::AppResult;
use endless_sdk::helper_client::Overrides;
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::rest_client::endless_api_types::UserTransaction;
use endless_sdk::rest_client::{PendingTransaction, Response, Transaction};
use endless_sdk::types::LocalAccount;

#[async_trait::async_trait]
pub trait AcctClientTrait {
    /// Recover Account, with or without sequence number
    async fn recover_account(
        &self,
        private_key: &str,
        with_sequence_number: bool,
    ) -> AppResult<LocalAccount>;

    /// Get Account Sequence Number
    async fn get_sequence_number(&self, account: &AccountAddress) -> AppResult<u64>;

    /// Set Latest Sequence Number
    async fn set_latest_sequence_number(&self, account: &mut LocalAccount) -> AppResult<()>;

    /// Query Endless Coins(EDS) Balance
    async fn balance_of(&self, owner: Owner) -> AppResult<u128>;

    /// Query Token Balance
    async fn token_balance_of(&self, owner: Owner, token: Token) -> AppResult<u128>;

    async fn faucet(
        &self,
        signer: &LocalAccount,
        receiver: AccountAddress,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>>;

    async fn faucet_wait_txn(
        &self,
        signer: &LocalAccount,
        receiver: AccountAddress,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Transaction>>;

    async fn simulate_transfer(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Vec<UserTransaction>>>;

    async fn transfer(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>>;

    async fn transfer_wait_txn(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Transaction>>;

    async fn simulate_transfer_token(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        token: Token,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Vec<UserTransaction>>>;

    async fn transfer_token(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        token: Token,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>>;

    async fn transfer_token_wait_txn(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        token: Token,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Transaction>>;
}

#[async_trait::async_trait]
impl AcctClientTrait for EnhancedClient {
    async fn recover_account(
        &self,
        private_key: &str,
        with_sequence_number: bool,
    ) -> AppResult<LocalAccount> {
        let mut acct = private_key.recover_account()?;
        if with_sequence_number {
            self.set_latest_sequence_number(&mut acct).await?;
        }
        Ok(acct)
    }

    async fn get_sequence_number(&self, account: &AccountAddress) -> AppResult<u64> {
        let (mn, fun, args) = ("account", "get_sequence_number", vec![account.to_bytes()?]);
        let args = ViewFnArgs::new(AccountAddress::ONE, mn, fun, args, vec![])?;
        self.view_fn(args, &EdsErr::GetAcctSeqNum, None).await
    }

    async fn set_latest_sequence_number(&self, account: &mut LocalAccount) -> AppResult<()> {
        let seq_num = self.get_sequence_number(&account.address()).await?;
        account.set_sequence_number(seq_num);
        Ok(())
    }

    async fn balance_of(&self, owner: Owner) -> AppResult<u128> {
        let (args, t_args) = (vec![owner.to_bytes()?], vec![]);
        let args = ViewFnArgs::new(AccountAddress::ONE, "endless_coin", "balance", args, t_args)?;
        self.view_fn(args, &EdsErr::EdsBalanceOf, None).await
    }

    async fn token_balance_of(&self, owner: Owner, token: Token) -> AppResult<u128> {
        let args = vec![owner.to_bytes()?, token.to_bytes()?];
        let t_args = vec!["0x1::fungible_asset::Metadata"];
        let (mun, fun) = ("primary_fungible_store", "balance");

        let args = ViewFnArgs::new(AccountAddress::ONE, mun, fun, args, t_args)?;
        self.view_fn(args, &EdsErr::TokenBalanceOf, None)
            .await
    }

    async fn faucet(
        &self,
        signer: &LocalAccount,
        receiver: AccountAddress,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>> {
        let (mn, fun, args) = ("faucet", "fund", vec![receiver.to_bytes()?]);
        let fn_args = EntryFnArgs::new(signer, AccountAddress::ONE, mn, fun, args, vec![])?
            .with_overrides(overrides);
        self.rest_client().entry_fun(fn_args).await
    }

    async fn faucet_wait_txn(
        &self,
        signer: &LocalAccount,
        receiver: AccountAddress,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Transaction>> {
        let res = self.faucet(signer, receiver, overrides).await?;
        self.wait_for_txn(res.inner()).await
    }

    async fn simulate_transfer(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Vec<UserTransaction>>> {
        let args = vec![to.to_bytes()?, amount.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer", from);

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, vec![])?
            .with_overrides(overrides);
        self.rest_client().simulate_fun(fn_args).await
    }

    async fn transfer(
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

    async fn transfer_wait_txn(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Transaction>> {
        let res = self.transfer(from, to, amount, overrides).await?;
        self.wait_for_txn(res.inner()).await
    }

    async fn simulate_transfer_token(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        token: Token,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Vec<UserTransaction>>> {
        let args = vec![to.to_bytes()?, amount.to_bytes()?, token.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer_coins", from);
        let t_args = vec!["0x1::fungible_asset::Metadata"];

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, t_args)?
            .with_overrides(overrides);
        self.rest_client().simulate_fun(fn_args).await
    }

    async fn transfer_token(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        token: Token,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<PendingTransaction>> {
        let args = vec![to.to_bytes()?, amount.to_bytes()?, token.to_bytes()?];
        let (mn, fun, owner) = ("endless_account", "transfer_coins", from);
        let t_args = vec!["0x1::fungible_asset::Metadata"];

        let fn_args = EntryFnArgs::new(owner, AccountAddress::ONE, mn, fun, args, t_args)?
            .with_overrides(overrides);
        self.rest_client().entry_fun(fn_args).await
    }

    async fn transfer_token_wait_txn(
        &self,
        from: &LocalAccount,
        to: AccountAddress,
        token: Token,
        amount: u128,
        overrides: Option<Overrides>,
    ) -> AppResult<Response<Transaction>> {
        let res = self
            .transfer_token(from, to, token, amount, overrides)
            .await?;
        self.wait_for_txn(res.inner()).await
    }
}
