use base_infra::result::AppResult;
use endless_client::client::EnhancedClient;
use endless_client::sdk_ext::account::LocalAccountExt;
use endless_client::sdk_ext::types::ViewFnArgs;
use endless_client::utils::account_ext::ToAccountAddress;
use endless_client::utils::bcs_ext::BcsExt;
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::rest_client::Response;
use endless_sdk::types::LocalAccount;

fn test_recover_account() -> AppResult<LocalAccount> {
    let account =
        "0x4e4d1a17673091a707d786004f6ba7f86ff41396062c42b1dbffd734af9334f3".recover_account()?;
    Ok(account)
}

base_infra::gen_impl_code_enum! {
     TestErr {
        GetUnlockedBalance = ("TST001", "Get unlocked balance failed"),

     }
}

// ido addr
// 9MprBvkH5jJnK1LaHnuBMAPBQwytF1qBUBvaC8R5vYT8
//
// 6ye6K1mHmCje3PLb38cFPx2xdqg4zyTLYdLUdTYrQdU8
//
// FSHZyivVsnbeS2JJrjM4fuPjv8a2jFB6DyHSyr1hEkP4
//
// 9VLUbDgGiAk8vif5vzycgAHxPCNxcr5SBhMgTbuQPbvv
//
// yHE9ZB3yjcq7BRkb2YVJLGeLAGn2iTn2EV8X64Qivci
//
// DDjpRZeMcr9VAUT2FKEKUkSEqEFxSqsYAzpUG3tquKe9
//
// 2n2xQgjeueV77Bv63vTJFZgADhaHQrshBpcsuFwckzga
//
// APW7VrGceBnosH74WhsHWDLKj7JgKrFYYknNb7ob5Ky4
//
// Bsbpaoai7xQzXFSona9H1PmKfoQAHgBzMYAZwgghYexx
//
// HWcPpjxKPsTwDbsGbzMj82jV4sb5RwHocGXsQ9tcF9sA

// unlocked_balance
async fn test_unlocked_balance(client: &EnhancedClient) -> AppResult<()> {
    let eds = "ENDLESSsssssssssssssssssssssssssssssssssssss".to_account_address()?;
    let recipient = "FUH7QsNDZmsFLEXrycR9pXQer2BSgGoUQ84b4RVJkWND".to_account_address()?;
    let args = vec![eds.to_bytes()?, recipient.to_bytes()?];

    let mod_addr = "0x1".to_account_address()?;
    let (mod_name, fn_name) = ("locking_coin_ex", "unlocked_balance");
    let args = ViewFnArgs::new(mod_addr, mod_name, fn_name, args, vec![])?;
    let res: u128 = client
        .view_fn(args, &TestErr::GetUnlockedBalance, None)
        .await?;
    println!("unlocked_balance: {}", res);
    Ok(())
}

/// Unlock amount and when to unlock.
#[derive(Debug, Clone, serde::Deserialize)]
struct UnlockAt {
    epoch: u64,
    amount: u128,
}

/// Unlocked token amount when and how much to unlock.
#[derive(Debug, Clone, serde::Deserialize)]
struct UnlockInfo {
    address: AccountAddress,
    unlocked: u128,
    unlock_list: Vec<UnlockAt>,
}
async fn test_get_unlock_info(client: &EnhancedClient) -> AppResult<()> {
    let eds = "ENDLESSsssssssssssssssssssssssssssssssssssss".to_account_address()?;
    let recipient = "FUH7QsNDZmsFLEXrycR9pXQer2BSgGoUQ84b4RVJkWND".to_account_address()?;
    let args = vec![eds.to_bytes()?, recipient.to_bytes()?];

    let mod_addr = "0x1".to_account_address()?;
    let (mod_name, fn_name) = ("locking_coin_ex", "get_unlock_info");
    let args = ViewFnArgs::new(mod_addr, mod_name, fn_name, args, vec![])?;
    let res: UnlockInfo = client
        .view_fn(args, &TestErr::GetUnlockedBalance, None)
        .await?;
    println!("get_unlock_info: {:?}", res);
    Ok(())
}

// staking_amount
async fn test_staking_amount(client: &EnhancedClient) -> AppResult<()> {
    let eds = "ENDLESSsssssssssssssssssssssssssssssssssssss".to_account_address()?;
    let recipient = "FUH7QsNDZmsFLEXrycR9pXQer2BSgGoUQ84b4RVJkWND".to_account_address()?;
    let args = vec![eds.to_bytes()?, recipient.to_bytes()?];

    let mod_addr = "0x1".to_account_address()?;
    let (mod_name, fn_name) = ("locking_coin_ex", "staking_amount");
    let args = ViewFnArgs::new(mod_addr, mod_name, fn_name, args, vec![])?;
    let res: u128 = client
        .view_fn(args, &TestErr::GetUnlockedBalance, None)
        .await?;
    println!("staking_amount: {}", res);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // recover account
    // let mut account = test_recover_account()?;

    let client = EnhancedClient::new_with_url_str("https://rpc-test.endless.link/v1")?;

    // unlocked_balance
    test_unlocked_balance(&client).await?;
    println!("---------------------------------------");

    // staking_amount
    test_staking_amount(&client).await?;
    println!("---------------------------------------");

    // get_unlock_info
    test_get_unlock_info(&client).await?;
    Ok(())
}
