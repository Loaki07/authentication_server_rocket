use argon2::{self, Config};
use dotenv::dotenv;
use color_eyre::Result;
use eyre::eyre;
use futures::compat::Future01CompatExt;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: String,
}

impl CryptoService {
    pub fn new() -> Self {
        dotenv().ok();
        let key_from_env: String = std::env::var("SECRET_KEY")
            .expect("Secret key for hashing not set")
            .into();
        Self { key: key_from_env }
    }

    #[instrument(skip(self, password))]
    pub async fn hash_password(&self, password: String) -> Result<String> {
        let config = Config::default();
        argon2::hash_encoded(&password.as_bytes(), &self.key.as_bytes(), &config)
            .map_err(|err| eyre!("Hashing error: {:?}", err))
    }

    #[instrument(skip(self, password, password_hash))]
    pub async fn verify_password(&self, password: String, password_hash: String) -> Result<bool> {
        let config = Config::default();
        argon2::verify_encoded(&password_hash, &password.as_bytes())
            .map_err(|err| eyre!("Verification Error: {:?}", err))
    }
}
