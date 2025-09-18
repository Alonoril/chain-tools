use crate::endless::error::EdsWltErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use bip39::{Language, Mnemonic};
use rand::{CryptoRng, RngCore};

/// Mnemonic word count options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordCount {
    /// 12 words (128 bits entropy)
    Twelve = 12,
    /// 15 words (160 bits entropy)
    Fifteen = 15,
    /// 18 words (192 bits entropy)
    Eighteen = 18,
    /// 21 words (224 bits entropy)
    TwentyOne = 21,
    /// 24 words (256 bits entropy)
    TwentyFour = 24,
}

impl WordCount {
    /// Get entropy bytes count for the word count
    pub fn entropy_bytes(self) -> usize {
        match self {
            WordCount::Twelve => 16,     // 128 bits
            WordCount::Fifteen => 20,    // 160 bits
            WordCount::Eighteen => 24,   // 192 bits
            WordCount::TwentyOne => 28,  // 224 bits
            WordCount::TwentyFour => 32, // 256 bits
        }
    }

    /// Get all available word counts
    pub fn all() -> [WordCount; 5] {
        [
            WordCount::Twelve,
            WordCount::Fifteen,
            WordCount::Eighteen,
            WordCount::TwentyOne,
            WordCount::TwentyFour,
        ]
    }
}

impl Default for WordCount {
    fn default() -> Self {
        WordCount::Twelve
    }
}

impl std::fmt::Display for WordCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} words", *self as u8)
    }
}

/// Mnemonic phrase generator with various options
pub struct PhraseGenerator {
    language: Language,
    word_count: WordCount,
}

impl Default for PhraseGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PhraseGenerator {
    /// Create a new phrase generator with default settings (English, 12 words)
    pub fn new() -> Self {
        Self {
            language: Language::English,
            word_count: WordCount::Twelve,
        }
    }

    /// Set the language for mnemonic generation
    pub fn with_language(self, language: Language) -> Self {
        Self {
            language,
            word_count: self.word_count,
        }
    }
    
    /// Set the word count for mnemonic generation
    pub fn with_word_count(self, word_count: WordCount) -> Self {
        Self {
            language: self.language,
            word_count,
        }
    }

    /// Generate a new mnemonic phrase using the system's cryptographic random number generator
    pub fn generate(&self) -> AppResult<String> {
        self.generate_with_rng(&mut rand::thread_rng())
    }

    /// Generate a new mnemonic phrase using a custom random number generator
    pub fn generate_with_rng<R>(&self, rng: &mut R) -> AppResult<String>
    where
        R: RngCore + CryptoRng,
    {
        let entropy_bytes = self.word_count.entropy_bytes();
        let mut entropy = vec![0u8; entropy_bytes];
        rng.fill_bytes(&mut entropy);

        let mnemonic = Mnemonic::from_entropy_in(self.language, &entropy)
            .map_err(map_err!(&EdsWltErr::InvalidMnemonic))?;

        Ok(mnemonic.to_string())
    }

    /// Generate a mnemonic phrase from specific entropy bytes
    pub fn from_entropy(&self, entropy: &[u8]) -> AppResult<String> {
        let expected_bytes = self.word_count.entropy_bytes();
        if entropy.len() != expected_bytes {
            return Err((&EdsWltErr::InvalidMnemonic).into());
        }

        let mnemonic = Mnemonic::from_entropy_in(self.language, entropy)
            .map_err(map_err!(&EdsWltErr::InvalidMnemonic))?;

        Ok(mnemonic.to_string())
    }

    /// Validate an existing mnemonic phrase
    pub fn validate(&self, phrase: &str) -> AppResult<bool> {
        match Mnemonic::parse_in_normalized(self.language, phrase) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get the current language setting
    pub fn language(&self) -> Language {
        self.language
    }

    /// Get the current word count setting
    pub fn word_count(&self) -> WordCount {
        self.word_count
    }
}

/// Convenience functions for quick mnemonic generation

/// Generate a 12-word English mnemonic phrase
pub fn generate_mnemonic() -> AppResult<String> {
    PhraseGenerator::new().generate()
}

/// Generate a mnemonic phrase with specific word count
pub fn generate_mnemonic_with_word_count(word_count: WordCount) -> AppResult<String> {
    PhraseGenerator::new()
        .with_word_count(word_count)
        .generate()
}

/// Generate a mnemonic phrase with specific language
pub fn generate_mnemonic_with_language(language: Language) -> AppResult<String> {
    PhraseGenerator::new().with_language(language).generate()
}

/// Generate a mnemonic phrase with custom settings
pub fn generate_mnemonic_custom(language: Language, word_count: WordCount) -> AppResult<String> {
    PhraseGenerator::new()
        .with_language(language)
        .with_word_count(word_count)
        .generate()
}

/// Validate a mnemonic phrase in English
pub fn validate_mnemonic(phrase: &str) -> AppResult<bool> {
    PhraseGenerator::new().validate(phrase)
}

/// Validate a mnemonic phrase in specific language
pub fn validate_mnemonic_with_language(phrase: &str, language: Language) -> AppResult<bool> {
    PhraseGenerator::new()
        .with_language(language)
        .validate(phrase)
}

/// Generate multiple mnemonic phrases for testing/comparison
pub fn generate_multiple_mnemonics(count: usize) -> AppResult<Vec<String>> {
    let generator = PhraseGenerator::new();
    let mut phrases = Vec::with_capacity(count);

    for _ in 0..count {
        phrases.push(generator.generate()?);
    }

    Ok(phrases)
}

/// Generate multiple mnemonic phrases with custom settings
pub fn generate_multiple_mnemonics_custom(
    count: usize,
    language: Language,
    word_count: WordCount,
) -> AppResult<Vec<String>> {
    let generator = PhraseGenerator::new()
        .with_language(language)
        .with_word_count(word_count);
    let mut phrases = Vec::with_capacity(count);

    for _ in 0..count {
        phrases.push(generator.generate()?);
    }

    Ok(phrases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count_entropy_bytes() {
        assert_eq!(WordCount::Twelve.entropy_bytes(), 16);
        assert_eq!(WordCount::Fifteen.entropy_bytes(), 20);
        assert_eq!(WordCount::Eighteen.entropy_bytes(), 24);
        assert_eq!(WordCount::TwentyOne.entropy_bytes(), 28);
        assert_eq!(WordCount::TwentyFour.entropy_bytes(), 32);
    }

    #[test]
    fn test_default_phrase_generator() {
        let generator = PhraseGenerator::new();
        assert_eq!(generator.language(), Language::English);
        assert_eq!(generator.word_count(), WordCount::Twelve);
    }

    #[test]
    fn test_phrase_generator_with_custom_settings() {
        let generator = PhraseGenerator::new()
            .with_language(Language::English)
            .with_word_count(WordCount::TwentyFour);

        assert_eq!(generator.language(), Language::English);
        assert_eq!(generator.word_count(), WordCount::TwentyFour);
    }

    #[test]
    fn test_generate_mnemonic() {
        let phrase = generate_mnemonic().expect("Failed to generate mnemonic");
        assert!(!phrase.is_empty());

        // Verify it's a valid 12-word phrase
        let words: Vec<&str> = phrase.split_whitespace().collect();
        assert_eq!(words.len(), 12);
    }

    #[test]
    fn test_generate_mnemonic_with_different_word_counts() {
        for word_count in WordCount::all() {
            let phrase =
                generate_mnemonic_with_word_count(word_count).expect("Failed to generate mnemonic");

            let words: Vec<&str> = phrase.split_whitespace().collect();
            assert_eq!(words.len(), word_count as usize);
        }
    }

    #[test]
    fn test_validate_mnemonic() {
        let phrase = generate_mnemonic().expect("Failed to generate mnemonic");
        let is_valid = validate_mnemonic(&phrase).expect("Failed to validate mnemonic");
        assert!(is_valid);

        // Test invalid phrase
        let invalid_phrase = "invalid phrase here";
        let is_invalid = validate_mnemonic(invalid_phrase).expect("Failed to validate mnemonic");
        assert!(!is_invalid);
    }

    #[test]
    fn test_from_entropy() {
        let generator = PhraseGenerator::new();
        let entropy = [1u8; 16]; // 16 bytes for 12 words

        let phrase = generator
            .from_entropy(&entropy)
            .expect("Failed to generate from entropy");
        assert!(!phrase.is_empty());

        // Should generate the same phrase with same entropy
        let phrase2 = generator
            .from_entropy(&entropy)
            .expect("Failed to generate from entropy");
        assert_eq!(phrase, phrase2);
    }

    #[test]
    fn test_generate_multiple_mnemonics() {
        let phrases =
            generate_multiple_mnemonics(3).expect("Failed to generate multiple mnemonics");
        assert_eq!(phrases.len(), 3);

        // All phrases should be different
        assert_ne!(phrases[0], phrases[1]);
        assert_ne!(phrases[1], phrases[2]);
        assert_ne!(phrases[0], phrases[2]);
    }

    #[test]
    fn test_word_count_display() {
        assert_eq!(WordCount::Twelve.to_string(), "12 words");
        assert_eq!(WordCount::TwentyFour.to_string(), "24 words");
    }
}
