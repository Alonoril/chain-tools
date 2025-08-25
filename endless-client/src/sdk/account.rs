use crate::error::EdsErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::crypto::ed25519::{Ed25519PrivateKey, Ed25519PublicKey};
use endless_sdk::types::LocalAccount;
use endless_sdk::types::transaction::authenticator::AuthenticationKey;

pub trait LocalAcctExt {
    fn recover_account(self) -> AppResult<LocalAccount>;
}

impl LocalAcctExt for Ed25519PrivateKey {
    fn recover_account(self) -> AppResult<LocalAccount> {
        let public_key = Ed25519PublicKey::from(&self);
        let akey = AuthenticationKey::ed25519(&public_key);
        let address = akey.account_address();
        Ok(LocalAccount::new(address, self, 0))
    }
}

impl LocalAcctExt for &str {
    fn recover_account(self) -> AppResult<LocalAccount> {
        let sk = self.strip_prefix("0x").unwrap_or(self);
        let sk_bytes = hex::decode(sk).map_err(map_err!(&EdsErr::InvalidHexPriKey))?;
        let esk = Ed25519PrivateKey::try_from(&sk_bytes[..])
            .map_err(map_err!(&EdsErr::ParseToEd25519Sk))?;
        esk.recover_account()
    }
}
