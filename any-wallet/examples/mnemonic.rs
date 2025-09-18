use any_wallet::{
    MnemonicWallet, MnemonicWalletGenerator, derive_wallet_batch,
    derive_wallet_batch_with_passphrase, derive_wallet_with_passphrase,
};
use base_infra::result::AppResult;
// use bip39::{Language, Mnemonic}; // Not needed for this example
use hex::encode as hex_encode;
use std::env;

fn main() -> AppResult<()> {
    let mut args = env::args().skip(1);
    let phrase = args.next().unwrap_or_else(|| {
        // Use test mnemonic for reproducible results
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        println!("Using test mnemonic: {phrase}");
        phrase.to_string()
    });
    let passphrase = args.next().unwrap_or_default();

    let generator = MnemonicWalletGenerator::new_with_passphrase(&phrase, &passphrase)?;

    println!("\n-- Single wallet derivation (index 0) --");
    let first_wallet = derive_wallet_with_passphrase(&phrase, &passphrase, 0)?;
    print_wallet(&first_wallet);

    println!("\n-- Batch derivation using the same mnemonic --");
    let batch = derive_wallet_batch_with_passphrase(&phrase, &passphrase, 1, 4)?;
    for wallet in &batch {
        print_wallet(wallet);
    }

    println!("\n-- Reusing generator for a large batch --");
    let generator_wallets = generator.derive_wallets(5, 3)?;
    for wallet in &generator_wallets {
        print_wallet(wallet);
    }

    println!(
        "\nVerify helper consistency: batch len (free fn) = {}, batch len (generator) = {}",
        batch.len(),
        generator_wallets.len()
    );

    // Bulk derivations
    let count = 16usize;
    let wallets = derive_wallet_batch(&phrase, 0, count)?;
    println!("\nDerived {count} wallets starting from index 0 using empty passphrase");
    println!(
        "First derived address: {}",
        wallets.first().unwrap().account_address()
    );
    println!("Last derived index: {}", wallets.last().unwrap().index());

    Ok(())
}

fn print_wallet(wallet: &MnemonicWallet) {
    let addr = wallet.account_address().to_hex_literal();
    let auth_key_hex = hex_encode(wallet.authentication_key().to_vec());
    let private_key_hex = hex_encode(wallet.private_key().to_bytes());
    println!(
        "index {:>3} | address {} | auth_key {} | priv_key {}",
        wallet.index(),
        addr,
        auth_key_hex,
        private_key_hex
    );
}
