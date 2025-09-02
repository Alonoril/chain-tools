use endless_client::client::account_client::AcctClientTrait;
use endless_client::client::types::Owner;
use endless_client::client::EnhancedClient;
use endless_sdk::types::LocalAccount;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let alice = LocalAccount::generate(&mut rand::rngs::OsRng);
    let bob = LocalAccount::generate(&mut rand::rngs::OsRng);

    // new client
    let client = EnhancedClient::new_with_url_str("https://rpc-test.endless.link/v1")?;

    // faucet
    client
        .faucet_wait_txn(&alice, alice.address(), None)
        .await?;
    client.faucet_wait_txn(&bob, bob.address(), None).await?;

    // alice balance
    let ali_bal = client.balance_of(Owner::new(&alice.address())).await?;
    println!("alice balance: {}", ali_bal);
    // bob balance
    let bob_bal = client.balance_of(Owner::new(&bob.address())).await?;
    println!("bob balance: {}", bob_bal);

    // simulate_transfer
    let res = client
        .simulate_transfer(&alice, bob.address(), 1000, None)
        .await?;
    println!(
        "simulate_transfer vm_status: {}",
        res.inner()[0].info.vm_status
    );

    // transfer
    let res = client
        .transfer_wait_txn(&alice, bob.address(), 1000, None)
        .await?;
    let info = res.inner().transaction_info()?;
    println!(
        "transfer txn_hash[{}] vm_status: {:?}",
        info.hash, info.vm_status
    );

    // alice balance
    let ali_bal = client.balance_of(Owner::new(&alice.address())).await?;
    println!("alice balance: {}", ali_bal);
    // bob balance
    let bob_bal = client.balance_of(Owner::new(&bob.address())).await?;
    println!("bob balance: {}", bob_bal);

    Ok(())
}
