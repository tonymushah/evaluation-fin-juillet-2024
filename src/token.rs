use std::{env, ops::Deref};

use dotenvy::dotenv;
use hmac::{digest::KeyInit, Hmac};
use http::header::AUTHORIZATION;
use jwt::VerifyWithKey;
use sha2::Sha256;

#[derive(Debug, Clone)]
pub struct ClientHmac(Hmac<Sha256>);

impl Deref for ClientHmac {
    type Target = Hmac<Sha256>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ClientHmac {
    fn extract(key: &'static str) -> ClientHmac {
        dotenv().ok();
        let client_hmac =
            env::var(key).unwrap_or_else(|_| panic!("Cannot find the `{key}` env variable"));
        Self(Hmac::new_from_slice(client_hmac.as_bytes()).unwrap())
    }
    pub fn extract_admin() -> ClientHmac {
        Self::extract("ADMINHMAC")
    }
    pub fn extract_client() -> ClientHmac {
        Self::extract("CLIENTHMAC")
    }
}

pub trait ExtractSessionData {
    fn get_current(&self, client_hmac: &ClientHmac) -> crate::Result<String>;
}

impl<T> ExtractSessionData for tonic::Request<T> {
    fn get_current(&self, client_hmac: &ClientHmac) -> crate::Result<String> {
        let metadata = self.metadata();
        let token = metadata
            .get(AUTHORIZATION.as_str())
            .ok_or(crate::Error::Forbidden)?;
        Ok(token.to_str()?.verify_with_key(client_hmac.deref())?)
    }
}
