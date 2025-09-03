use endless_client::client::EnhancedClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = EnhancedClient::new_with_url_str("https://rpc-test.endless.link/v1")?;

    Ok(())
}
