use base_infra::result::AppResult;
use endless_client::client::EnhancedClient;
use endless_client::client::account_client::AcctClientTrait;
use endless_client::client::types::{Owner, Token};
use endless_sdk::types::LocalAccount;
use chain_types::endless::eds_addr_ext::ToEdsAddr;

async fn test_recover_account(client: &EnhancedClient) -> AppResult<LocalAccount> {
    let account = client
        .recover_account(
            "0x4e4d1a17673091a707d786004f6ba7f86ff41396062c42b1dbffd734af9334f3",
            true,
        )
        .await?;
    Ok(account)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // new client
    let client = EnhancedClient::new_with_url_str("https://rpc-test.endless.link/v1")?;

    let signer = test_recover_account(&client).await?;
    println!("owner address: {}", signer.address().to_bs58_string());
    let owner = Owner::new(&signer.address());
    let usdt = "USDH437BQjeVRzACuLiJQ6Bc9WaBSe1tWxcaNtJoa1s".to_eds_addr()?;
    let usdt_token = Token::new(&usdt);
    let receiver = "AKe6can1eJW6sZ7WsTkK8pNVxheeQHMEe8VX81Cu4bJT".to_eds_addr()?;

    //simulate_transfer_token
    let res = client
        .simulate_transfer_token(&signer, receiver, usdt_token, 10, None)
        .await?;
    println!(
        "simulate_transfer_token vm_status: {}",
        res.inner()[0].info.vm_status
    );

    // transfer USDT
    let balance = client.token_balance_of(owner, usdt_token).await?;
    println!("before transfer USDT balance: {}", balance);

    let res = client
        .transfer_token_wait_txn(&signer, receiver, usdt_token, 10, None)
        .await?;
    let info = res.inner().transaction_info()?;
    println!("txn hash: {}, success: {}", info.hash, info.success);

    // USDT balance
    let balance = client.token_balance_of(owner, usdt_token).await?;
    println!("after transfer USDT balance: {}", balance);
    Ok(())
}
