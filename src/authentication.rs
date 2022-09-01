use anyhow::Result;
use bsv::{hash::Hash, AESAlgorithms, PBKDF2Hashes, AES, KDF};
use serde::{Deserialize, Serialize};
use std::str;

pub struct Authentication {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenPayload {
    user: AuthTokenUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenUser {
    id: String,
}

macro_rules! expect_two {
    ($iter:expr) => {{
        let mut i = $iter;
        match (i.next(), i.next(), i.next()) {
            (Some(first), Some(second), None) => (first, second),
            _ => return Err(anyhow::anyhow!("Invalid Token")),
        }
    }};
}

impl AuthToken {
    pub fn new(token: String) -> Result<AuthToken> {
        let user = AuthToken::decode(&token)?;
        Ok(AuthToken {
            token,
            user_id: user.id,
        })
    }

    pub fn decode(token: &String) -> Result<AuthTokenUser> {
        let (_, message) = expect_two!(token.rsplitn(2, '.'));
        let (payload, _) = expect_two!(message.rsplitn(2, '.'));
        let decoded = base64::decode_config(payload, base64::URL_SAFE_NO_PAD)?;
        let utf8 = str::from_utf8(&decoded)?.to_string();
        let token: AuthTokenPayload = serde_json::from_str(&utf8)?;
        Ok(token.user)
    }
}

pub struct AuthenticationCipher {
    pub email_hash: String,
    pub cipher: String,
    pub password_hash: String,
    pub cipher_hash: Vec<u8>,
}

impl Authentication {
    pub fn get_cipher(email: String, password: String) -> AuthenticationCipher {
        let email_hash = Hash::sha_256(email.as_bytes()).to_hex();
        let cipher = KDF::pbkdf2(
            password.as_bytes().into(),
            Some(email_hash.as_bytes().into()),
            PBKDF2Hashes::SHA256,
            10000,
            32,
        )
        .get_hash();

        let password_hash = Hash::sha_256(cipher.to_hex().as_bytes()).to_hex();

        let response = AuthenticationCipher {
            email_hash,
            password_hash,
            cipher: cipher.to_hex(),
            cipher_hash: cipher.to_bytes(),
        };

        return response;
    }
}

impl AuthenticationCipher {
    pub fn decrypt_mnemonic(&self, encrypted_mnemonic: String) -> Option<String> {
        let iv = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

        let decrypted = match AES::decrypt(
            &self.cipher_hash,
            iv,
            encrypted_mnemonic.as_bytes(),
            AESAlgorithms::AES128_CTR,
        ) {
            Ok(v) => v,
            Err(_) => return Some("error decrypting".to_string()),
        };

        let utf8 = match str::from_utf8(&decrypted) {
            Ok(v) => v,
            Err(_) => return Some("error utf8".to_string()),
        };

        return Some(utf8.into());
    }
}
