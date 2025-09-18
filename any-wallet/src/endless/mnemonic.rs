use crate::endless::error::EdsWltErr;
use base_infra::result::AppResult;
use base_infra::{else_err, map_err};
use bip39::{Language, Mnemonic};
use ed25519_dalek_bip32::{ChildIndex, ExtendedSecretKey};
use endless_sdk::crypto::ed25519::{Ed25519PrivateKey, Ed25519PublicKey};
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::types::transaction::authenticator::AuthenticationKey;
use std::convert::TryFrom;

const APTOS_COIN_TYPE: u32 = 637;
const HARDENED_PATH_PREFIX: [u32; 4] = [44, APTOS_COIN_TYPE, 0, 0];

/// Generated wallet information derived from a mnemonic phrase.
#[derive(Debug)]
pub struct MnemonicWallet {
    index: u32,
    public_key: Ed25519PublicKey,
    private_key: Ed25519PrivateKey,
}

impl MnemonicWallet {
    /// Returns the derivation index used for this wallet.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Returns the Endless account address associated with this wallet.
    pub fn account_address(&self) -> AccountAddress {
        self.authentication_key().account_address()
    }

    /// Returns a reference to the derived Endless private key.
    pub fn private_key(&self) -> &Ed25519PrivateKey {
        &self.private_key
    }

    /// Consumes the wallet and returns the derived Endless private key.
    pub fn into_private_key(self) -> Ed25519PrivateKey {
        self.private_key
    }

    /// Computes the authentication key corresponding to the private key.
    pub fn authentication_key(&self) -> AuthenticationKey {
        AuthenticationKey::ed25519(&self.public_key)
    }
}

/// Helper for repeatedly deriving wallets from the same mnemonic.
pub struct MnemonicWalletGenerator {
    root: ExtendedSecretKey,
}

impl MnemonicWalletGenerator {
    /// Builds a generator from the given mnemonic using an empty passphrase.
    pub fn new(phrase: &str) -> AppResult<Self> {
        Self::new_with_passphrase(phrase, "")
    }

    /// Builds a generator from the given mnemonic and passphrase.
    pub fn new_with_passphrase(phrase: &str, passphrase: &str) -> AppResult<Self> {
        let mnemonic = Mnemonic::parse_in_normalized(Language::English, phrase)
            .map_err(map_err!(&EdsWltErr::InvalidMnemonic))?;
        let seed = mnemonic.to_seed(passphrase);
        let root = ExtendedSecretKey::from_seed(&seed).map_err(map_err!(&EdsWltErr::SeedDerive))?;
        Ok(Self { root })
    }

    /// Derives a single wallet at the provided index.
    pub fn derive_wallet(&self, index: u32) -> AppResult<MnemonicWallet> {
        self.derive_wallet_from_root(index)
    }

    /// Derives `count` wallets starting from `start_index` (inclusive).
    pub fn derive_wallets(&self, start_index: u32, count: usize) -> AppResult<Vec<MnemonicWallet>> {
        self.derive_wallets_from_root(start_index, count)
    }

    fn derive_wallets_from_root(
        &self,
        start_idx: u32,
        count: usize,
    ) -> AppResult<Vec<MnemonicWallet>> {
        let mut wallets = Vec::with_capacity(count);
        let err = &EdsWltErr::IndexOverflow;

        for offset in 0..count {
            let offset = u32::try_from(offset).map_err(map_err!(err))?;
            let index = start_idx.checked_add(offset).ok_or_else(else_err!(err))?;
            wallets.push(self.derive_wallet_from_root(index)?);
        }
        Ok(wallets)
    }

    fn derive_wallet_from_root(&self, index: u32) -> AppResult<MnemonicWallet> {
        let path = derivation_path(index);
        let child = self
            .root
            .derive(&path)
            .map_err(map_err!(&EdsWltErr::ChildDerive))?;

        let secret = child.secret_key.to_bytes();
        let private_key = Ed25519PrivateKey::try_from(secret.as_ref())
            .map_err(map_err!(&EdsWltErr::PrivateKey))?;

        Ok(MnemonicWallet {
            index,
            public_key: Ed25519PublicKey::from(&private_key),
            private_key,
        })
    }
}

/// Derives a single Endless wallet from the provided mnemonic using an empty passphrase.
pub fn derive_wallet(phrase: &str, index: u32) -> AppResult<MnemonicWallet> {
    MnemonicWalletGenerator::new(phrase)?.derive_wallet(index)
}

/// Derives a single Endless wallet from the provided mnemonic and passphrase.
pub fn derive_wallet_with_passphrase(
    phrase: &str,
    passphrase: &str,
    index: u32,
) -> AppResult<MnemonicWallet> {
    MnemonicWalletGenerator::new_with_passphrase(phrase, passphrase)?.derive_wallet(index)
}

/// Derives `count` Endless wallets starting from index `start_index` using an empty passphrase.
pub fn batch_derive_wallet(
    phrase: &str,
    start_index: u32,
    count: usize,
) -> AppResult<Vec<MnemonicWallet>> {
    MnemonicWalletGenerator::new(phrase)?.derive_wallets(start_index, count)
}

/// Derives `count` Endless wallets starting from index `start_index` with the provided passphrase.
pub fn batch_derive_wallet_with_passphrase(
    phrase: &str,
    passphrase: &str,
    start_index: u32,
    count: usize,
) -> AppResult<Vec<MnemonicWallet>> {
    MnemonicWalletGenerator::new_with_passphrase(phrase, passphrase)?
        .derive_wallets(start_index, count)
}

fn derivation_path(index: u32) -> [ChildIndex; 5] {
    [
        ChildIndex::Hardened(HARDENED_PATH_PREFIX[0]),
        ChildIndex::Hardened(HARDENED_PATH_PREFIX[1]),
        ChildIndex::Hardened(HARDENED_PATH_PREFIX[2]),
        ChildIndex::Hardened(HARDENED_PATH_PREFIX[3]),
        ChildIndex::Hardened(index),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_infra::result::AppError;

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

    #[test]
    fn derive_single_wallet_matches_batch() {
        let wallet = derive_wallet(TEST_MNEMONIC, 0).expect("derive single wallet");
        let batch = batch_derive_wallet(TEST_MNEMONIC, 0, 1).expect("derive wallet batch");
        assert_eq!(batch.len(), 1);
        let batch_wallet = &batch[0];
        assert_eq!(wallet.index(), batch_wallet.index());
        assert_eq!(wallet.account_address(), batch_wallet.account_address());
        assert_eq!(
            wallet.private_key().to_bytes(),
            batch_wallet.private_key().to_bytes()
        );
    }

    #[test]
    fn derive_multiple_wallets_in_sequence() {
        let wallets = batch_derive_wallet(TEST_MNEMONIC, 0, 3).expect("derive wallets");
        assert_eq!(wallets.len(), 3);
        for (expected_index, wallet) in wallets.iter().enumerate() {
            assert_eq!(u32::try_from(expected_index).unwrap(), wallet.index());
        }
    }

    #[test]
    fn derive_wallet_overflow_guard() {
        let err = batch_derive_wallet(TEST_MNEMONIC, u32::MAX, 2).unwrap_err();
        match err {
            AppError::ErrCode(code) | AppError::Anyhow(code, _) => {
                assert_eq!(code.code(), "WLT005")
            }
            #[cfg(feature = "http")]
            AppError::HttpErr(code, _) => assert_eq!(code.code(), "WLT005"),
        }
    }
}
