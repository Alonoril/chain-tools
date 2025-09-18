use any_wallet::endless::phrase::{
    generate_mnemonic, generate_mnemonic_custom, generate_mnemonic_with_word_count,
    generate_multiple_mnemonics, validate_mnemonic, PhraseGenerator, WordCount,
};
use base_infra::result::AppResult;
use bip39::Language;

fn main() -> AppResult<()> {
    println!("=== Automatic Mnemonic Phrase Generation Demo ===\n");

    // 1. Generate a simple 12-word mnemonic
    println!("1. Generate a default 12-word English mnemonic:");
    let simple_mnemonic = generate_mnemonic()?;
    println!("   {}", simple_mnemonic);
    
    // Validate it
    let is_valid = validate_mnemonic(&simple_mnemonic)?;
    println!("   Valid: {}\n", is_valid);

    // 2. Generate mnemonics with different word counts
    println!("2. Generate mnemonics with different word counts:");
    for word_count in WordCount::all() {
        let mnemonic = generate_mnemonic_with_word_count(word_count)?;
        let word_count_actual = mnemonic.split_whitespace().count();
        println!("   {} -> {} words: {}", 
            word_count, 
            word_count_actual, 
            if mnemonic.len() > 80 { 
                format!("{}...", &mnemonic[..77])
            } else { 
                mnemonic 
            }
        );
    }
    println!();

    // 3. Using the PhraseGenerator for more control
    println!("3. Using PhraseGenerator for advanced configuration:");
    
    let generator = PhraseGenerator::new()
        .with_word_count(WordCount::TwentyFour);
    
    let custom_mnemonic = generator.generate()?;
    println!("   24-word mnemonic: {}...", &custom_mnemonic[..50]);
    println!("   Language: {:?}", generator.language());
    println!("   Word count: {}", generator.word_count());
    println!();

    // 4. Generate from specific entropy (deterministic)
    println!("4. Generate from specific entropy (deterministic):");
    let entropy = [42u8; 16]; // 16 bytes for 12 words
    let entropy_generator = PhraseGenerator::new().with_word_count(WordCount::Twelve);
    let deterministic_mnemonic1 = entropy_generator.from_entropy(&entropy)?;
    let deterministic_mnemonic2 = entropy_generator.from_entropy(&entropy)?;
    
    println!("   First generation:  {}", deterministic_mnemonic1);
    println!("   Second generation: {}", deterministic_mnemonic2);
    println!("   Same result: {}", deterministic_mnemonic1 == deterministic_mnemonic2);
    println!();

    // 5. Generate multiple mnemonics for comparison
    println!("5. Generate multiple mnemonics for comparison:");
    let multiple_mnemonics = generate_multiple_mnemonics(3)?;
    for (i, mnemonic) in multiple_mnemonics.iter().enumerate() {
        println!("   Mnemonic {}: {}", i + 1, mnemonic);
    }
    println!();

    // 6. Custom language and word count combination
    println!("6. Custom settings (English, 18 words):");
    let custom_full = generate_mnemonic_custom(Language::English, WordCount::Eighteen)?;
    println!("   {}", custom_full);
    
    let is_valid = validate_mnemonic(&custom_full)?;
    println!("   Valid: {}", is_valid);
    println!();

    // 7. Demonstrate entropy byte requirements
    println!("7. Entropy byte requirements for different word counts:");
    for word_count in WordCount::all() {
        println!("   {} needs {} bytes of entropy", word_count, word_count.entropy_bytes());
    }
    println!();

    // 8. Show validation with invalid mnemonic
    println!("8. Validation test:");
    let valid_mnemonic = generate_mnemonic()?;
    let invalid_mnemonic = "this is not a valid mnemonic phrase at all";
    
    println!("   Valid mnemonic:   {} -> {}", 
        if valid_mnemonic.len() > 50 { format!("{}...", &valid_mnemonic[..47]) } else { valid_mnemonic.clone() },
        validate_mnemonic(&valid_mnemonic)?
    );
    println!("   Invalid mnemonic: {} -> {}", 
        invalid_mnemonic, 
        validate_mnemonic(invalid_mnemonic)?
    );

    println!("\n=== Demo completed successfully! ===");
    Ok(())
}