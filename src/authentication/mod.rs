use wasm_bindgen::{prelude::*, JsValue};
use serde::*;
use bsv_wasm::{ PBKDF2Hashes, hash::Hash, KDF, AES };

#[wasm_bindgen]
pub struct Authentication {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct AuthenticationCipher {
    email_hash: String,
    cipher: String,
    password_hash: String
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Authentication {
    #[wasm_bindgen(js_name = getCipher)]
    pub fn get_cipher(email: String, password: String) -> JsValue {
        let email_hash = Hash::sha_256(email.as_bytes()).to_hex();
        let cipher = KDF::pbkdf2(password.as_bytes().into(), Some(email_hash.as_bytes().into()), PBKDF2Hashes::SHA256, 10000, 32).get_hash().to_hex();
        let password_hash = Hash::sha_256(cipher.as_bytes()).to_hex();

        let response = AuthenticationCipher { email_hash, password_hash, cipher };
        return JsValue::from_serde(&response).unwrap()
    }

    #[wasm_bindgen(js_name = decryptMnemonic)]
    pub fn decrypt_mnemonic(encryptedMnemonic: String, cipher: String) -> JsValue {
        return JsValue::from_serde(&false).unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct AuthenticationCipher {
    pub email_hash: String,
    pub cipher: String,
    pub password_hash: String
}

#[cfg(not(target_arch = "wasm32"))]
impl Authentication {
    pub fn get_cipher(email: String, password: String) -> AuthenticationCipher {
        let email_hash = Hash::sha_256(email.as_bytes()).to_hex();
        let cipher = KDF::pbkdf2(password.as_bytes().into(), Some(email_hash.as_bytes().into()), PBKDF2Hashes::SHA256, 10000, 32).get_hash().to_hex();
        let password_hash = Hash::sha_256(cipher.as_bytes()).to_hex();

        let response = AuthenticationCipher { email_hash, password_hash, cipher };
        return response;
    }
}
