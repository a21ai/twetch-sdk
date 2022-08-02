use bsv::{hash::Hash, AESAlgorithms, PBKDF2Hashes, AES, KDF};
use std::str;

pub struct Authentication {}

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
