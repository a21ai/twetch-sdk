use twetch_sdk::authentication;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Authentication(authentication::Authentication);

#[wasm_bindgen]
pub struct AuthToken(authentication::AuthToken);

#[wasm_bindgen(js_name = AuthenticationCipher)]
pub struct AuthenticationCipher(authentication::AuthenticationCipher);

#[wasm_bindgen]
impl AuthenticationCipher {
    #[wasm_bindgen(js_name = getEmailHash)]
    pub fn get_email_hash(&self) -> String {
        self.0.email_hash.clone()
    }

    #[wasm_bindgen(js_name = getPasswordHash)]
    pub fn get_password_hash(&self) -> String {
        self.0.password_hash.clone()
    }

    #[wasm_bindgen(js_name = getCipher)]
    pub fn get_cipher(&self) -> String {
        self.0.cipher.clone()
    }

    #[wasm_bindgen(js_name = decryptMnemonic)]
    pub fn decrypt_mnemonic(&self, encrypted_mnemonic: String) -> Option<String> {
        authentication::AuthenticationCipher::decrypt_mnemonic(&self.0, encrypted_mnemonic)
    }
}

#[wasm_bindgen]
impl Authentication {
    #[wasm_bindgen(js_name = getCipher)]
    pub fn get_cipher(email: String, password: String) -> AuthenticationCipher {
        AuthenticationCipher(authentication::Authentication::get_cipher(email, password))
    }
}
