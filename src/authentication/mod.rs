use wasm_bindgen::{prelude::*, JsValue, throw_str};
use serde::*;
use bsv_wasm::{ PBKDF2Hashes, hash::Hash, KDF, AES, AESAlgorithms };
use hex::FromHexError;
use hex::*;

#[wasm_bindgen]
pub struct Authentication {}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationCipher {
    email_hash: String,
    cipher: String,
    password_hash: String
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

impl Authentication {
    //pub fn decrypt_mnemonic_impl(cipher: String, encryptedMnemonic: String) -> Result<Vec<u8>, FromHexError> {
        //let key = hex::decode(cipher)?;
        //let iv = hex::decode("00000000000000000000000000000001")?;
        //let message = hex::decode(encryptedMnemonic)?;

        //return AES::decrypt(&key, &iv, &message, AESAlgorithms::AES256_CTR)

        ////return message;
    //}
}

#[wasm_bindgen]
impl Authentication {
    #[wasm_bindgen(js_name = getCipher)]
    pub fn get_cipher(email: String, password: String) -> AuthenticationCipher {
        let email_hash = Hash::sha_256(email.as_bytes()).to_hex();
        let cipher = KDF::pbkdf2(password.as_bytes().into(), Some(email_hash.as_bytes().into()), PBKDF2Hashes::SHA256, 10000, 32).get_hash().to_hex();
        let password_hash = Hash::sha_256(cipher.as_bytes()).to_hex();

        let response = AuthenticationCipher { email_hash, password_hash, cipher };
        return response;
    }

    //#[wasm_bindgen(js_name = decrypteMnemonic)]
    //pub fn decrypt_mnemonic(cipher: String, encryptedMnemonic: String) -> Result<Vec<u8>, JsValue> {
        //match Authentication::decrypt_mnemonic_impl(cipher, encryptedMnemonic) {
            //Ok(v) => Ok(v),
            //Err(e) => throw_str(&e.to_string()),
        //}
    //}
}
