use chain_types::endless::eds_addr_ext::ToEdsAddr;
use endless_client::client::EnhancedClient;
use endless_client::utils::txn_util::build_move_struct_tag;
use endless_sdk::rest_client::endless_api_types::MoveStructTag;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // new client
    let client = EnhancedClient::new_with_url_str("https://rpc-test.endless.link/v1")?;

    test_get_events_by_hash(&client).await?;
    Ok(())
}

async fn test_get_events_by_hash(client: &EnhancedClient) -> anyhow::Result<()> {
    let brg_addr = "BLiLNS4g2Xoz2FUuwxiufHKLu79xorpxYBdqREocsTiE".to_eds_addr()?;
    let brg_finish: MoveStructTag =
        build_move_struct_tag(brg_addr, "execute", "BridgeFinish", vec![])?;

    let pool_addr = "HMiZjh2Av7Ttp1CGEpKK2Pv8Vrftc94zniy8baKtrT3i".to_eds_addr()?;
    let swap: MoveStructTag =
        build_move_struct_tag(pool_addr, "liquidity_pool", "SwapEvent", vec![])?;

    let events = client
        .get_txn_events_by_hash(
            "9KEBtm19g4C8gCbM8yoQpND26EfdK8fq8c4AG11gMD8F",
            vec![brg_finish, swap],
        )
        .await?;
    println!("{:?}", events);
    Ok(())
}

async fn test_get_txn_by_hash(client: &EnhancedClient) -> anyhow::Result<()> {
    let ss = client
        .get_txn_by_hash("9KEBtm19g4C8gCbM8yoQpND26EfdK8fq8c4AG11gMD8F")
        .await?;
    println!("{:?}", ss);
    Ok(())
}
