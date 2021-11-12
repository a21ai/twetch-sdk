use bsv_wasm::{hash::Hash, AESAlgorithms, PBKDF2Hashes, AES, KDF};
use js_sys::decode_uri_component;
use serde::*;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

pub struct Authentication {}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationCipher {
    email_hash: String,
    cipher: String,
    password_hash: String,
    cipher_hash: Vec<u8>,
}

#[wasm_bindgen]
impl AuthenticationCipher {
    #[wasm_bindgen(js_name = getEmailHash)]
    pub fn get_email_hash(&self) -> String {
        self.email_hash.clone()
    }

    #[wasm_bindgen(js_name = getPasswordHash)]
    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }

    #[wasm_bindgen(js_name = getCipher)]
    pub fn get_cipher(&self) -> String {
        self.cipher.clone()
    }
}

#[wasm_bindgen]
impl Authentication {
    #[wasm_bindgen(js_name = getCipher)]
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

#[wasm_bindgen]
impl AuthenticationCipher {
    #[wasm_bindgen(js_name = decryptMnemonic)]
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

        //let decoded_utf8 = match decode_uri_component(&utf8) {
        //Ok(v) => v,
        //Err(_) => return Some("error decode uri".to_string()),
        //};

        return Some(utf8.into());
    }
}
