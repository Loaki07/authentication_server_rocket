use argonautica::{Hasher, Verifier};
use dotenv::dotenv;
use color_eyre::Result;
use eyre::eyre;
use futures::compat::Future01CompatExt;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {
    pub fn new() -> Self {
        dotenv().ok();
        let key_from_env: Arc<String> = std::env::var("SECRET_KEY")
            .expect("Secret key for hashing not set")
            .into();
        Self { key: key_from_env }
    }

    #[instrument(skip(self, password))]
    pub async fn hash_password(&self, password: String) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Hassing error: {:?}", err))
    }

    #[instrument(skip(self, password, password_hash))]
    pub async fn verify_password(&self, password: String, password_hash: String) -> Result<bool> {
        Verifier::default()
            .with_hash(password_hash)
            .with_password(password)
            .with_secret_key(&*self.key)
            .verify()
            .map_err(|err| eyre!("Verification Error: {:?}", err))
    }
}
