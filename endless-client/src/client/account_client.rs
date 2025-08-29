use crate::client::EnhancedClient;
use crate::client::types::{Owner, Token};
use crate::error::EdsErr;
use crate::sdk_ext::account::LocalAccountExt;
use crate::sdk_ext::types::ViewFnArgs;
use crate::utils::bcs_ext::BcsExt;
use base_infra::result::AppResult;
use endless_sdk::move_types::account_address::AccountAddress;
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
        self.view_fn_inner(args, &EdsErr::GetAcctSeqNum, None).await
    }

    async fn set_latest_sequence_number(&self, account: &mut LocalAccount) -> AppResult<()> {
        let seq_num = self.get_sequence_number(&account.address()).await?;
        account.set_sequence_number(seq_num);
        Ok(())
    }

    async fn balance_of(&self, owner: Owner) -> AppResult<u128> {
        let (args, t_args) = (vec![owner.to_bytes()?], vec![]);
        let args = ViewFnArgs::new(AccountAddress::ONE, "endless_coin", "balance", args, t_args)?;
        self.view_fn_inner(args, &EdsErr::EdsBalanceOf, None).await
    }

    async fn token_balance_of(&self, owner: Owner, token: Token) -> AppResult<u128> {
        let args = vec![owner.to_bytes()?, token.to_bytes()?];
        let t_args = vec!["0x1::fungible_asset::Metadata"];
        let (mun, fun) = ("primary_fungible_store", "balance");

        let args = ViewFnArgs::new(AccountAddress::ONE, mun, fun, args, t_args)?;
        self.view_fn_inner(args, &EdsErr::TokenBalanceOf, None)
            .await
    }
}
