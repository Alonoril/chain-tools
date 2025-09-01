use base_infra::result::AppResult;
use endless_client::client::EnhancedClient;
use endless_client::client::account_client::AcctClientTrait;
use endless_client::client::types::{Owner, Token};
use endless_client::sdk_ext::account::LocalAccountExt;
use endless_client::utils::account_ext::ToAccountAddress;
use endless_sdk::crypto::SigningKey;
use endless_sdk::crypto::hash::CryptoHash;
use endless_sdk::crypto::test_utils::TestEndlessCrypto;
use endless_sdk::types::LocalAccount;

fn test_recover_account() -> AppResult<LocalAccount> {
    let account =
        "0x4e4d1a17673091a707d786004f6ba7f86ff41396062c42b1dbffd734af9334f3".recover_account()?;
    Ok(account)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // recover account
    let mut account = test_recover_account()?;

    // new client
    let client = EnhancedClient::new_with_url_str("https://rpc-test.endless.link/v1")?;

    // set last sequence number
    println!("before set sequence_number: {}", account.sequence_number());
    client.set_latest_sequence_number(&mut account).await?;
    println!("after set sequence_number: {}", account.sequence_number());

    // get account sequence number
    let seq_num = client.get_sequence_number(&account.address()).await?;
    println!("get sequence_number: {seq_num}");

    // get account resource
    let res = client
        .get_client()
        .get_account_bcs(account.address())
        .await?;
    println!("{:?}", res.inner());

    // EDS balance
    let balance = client.balance_of(Owner::new(&account.address())).await?;
    println!("EDS balance: {}", balance);

    // Token balance
    let usdc = "8iSN2eUjbHV9jq5TLtBQYU4tLozcLnSGiN4HFy4u9WZw".to_account_address()?;
    let usdc_balance = client
        .token_balance_of(Owner::new(&account.address()), Token::new(&usdc))
        .await?;
    println!("USDC balance: {}", usdc_balance);

    // sign message
    let message = TestEndlessCrypto("AbC".to_string());
    println!("message hash {}", message.hash());
    let signature = account.private_key().sign(&message)?;
    println!("signature: {}", signature);
    Ok(())
}
