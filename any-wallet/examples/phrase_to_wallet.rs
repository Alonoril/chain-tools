use any_wallet::endless::mnemonic::derive_wallet;
use any_wallet::endless::phrase::{generate_mnemonic, generate_mnemonic_with_word_count, WordCount};
use base_infra::result::AppResult;
use hex::encode as hex_encode;

fn main() -> AppResult<()> {
    println!("=== Automatic Phrase Generation to Wallet Demo ===\n");

    // 1. Generate a mnemonic and derive the first wallet
    println!("1. Generate 12-word mnemonic and derive first wallet:");
    let mnemonic = generate_mnemonic()?;
    println!("   Mnemonic: {}", mnemonic);
    
    let wallet = derive_wallet(&mnemonic, 0)?;
    println!("   Index: {}", wallet.index());
    println!("   Address: {}", wallet.account_address().to_bs58_string());
    println!("   Auth Key: {}", hex_encode(wallet.authentication_key().to_vec()));
    println!("   Private Key: {}", hex_encode(wallet.private_key().to_bytes()));
    println!();

    // 2. Generate different word count mnemonics and derive wallets
    println!("2. Compare wallets from different entropy lengths:");
    let word_counts = [WordCount::Twelve, WordCount::TwentyFour];
    
    for word_count in word_counts {
        let mnemonic = generate_mnemonic_with_word_count(word_count)?;
        let wallet = derive_wallet(&mnemonic, 0)?;
        
        println!("   {} - Address: {}", 
            word_count,
            wallet.account_address().to_bs58_string()
        );
        println!("      Mnemonic: {}{}",
            if mnemonic.len() > 60 { format!("{}...", &mnemonic[..57]) } else { mnemonic },
            ""
        );
    }
    println!();

    // 3. Generate multiple wallets from the same mnemonic
    println!("3. Generate multiple wallets from same mnemonic:");
    let master_mnemonic = generate_mnemonic()?;
    println!("   Master mnemonic: {}", master_mnemonic);
    
    for i in 0..3 {
        let wallet = derive_wallet(&master_mnemonic, i)?;
        println!("   Wallet {}: {}", i, wallet.account_address().to_bs58_string());
    }
    println!();

    // 4. Show that same mnemonic always generates same wallets
    println!("4. Deterministic wallet generation test:");
    let test_mnemonic = generate_mnemonic()?;
    println!("   Test mnemonic: {}", test_mnemonic);
    
    let wallet1 = derive_wallet(&test_mnemonic, 0)?;
    let wallet2 = derive_wallet(&test_mnemonic, 0)?;
    
    println!("   First derivation:  {}", wallet1.account_address().to_bs58_string());
    println!("   Second derivation: {}", wallet2.account_address().to_bs58_string());
    println!("   Same address: {}", wallet1.account_address() == wallet2.account_address());
    println!();

    // 5. Security best practices demonstration
    println!("5. Security best practices:");
    let secure_mnemonic = generate_mnemonic_with_word_count(WordCount::TwentyFour)?;
    let wallet = derive_wallet(&secure_mnemonic, 0)?;
    
    println!("   ✓ Use 24-word mnemonic for maximum security");
    println!("   ✓ Generated with cryptographically secure randomness");
    println!("   ✓ BIP39 compliant mnemonic phrase");
    println!("   ✓ BIP32/SLIP-10 HD wallet derivation");
    println!("   Example secure address: {}", wallet.account_address().to_bs58_string());
    
    println!("\n=== Demo completed successfully! ===");
    println!("💡 Remember to store your mnemonic phrases securely!");
    Ok(())
}