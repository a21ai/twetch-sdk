use bsv_wasm::{ExtendedPrivateKey, ECIESCiphertext, ECIES};
use serde::*;
use wasm_bindgen::prelude::*;
use base64;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatConversation {}

#[wasm_bindgen]
impl ChatConversation {
    #[wasm_bindgen(js_name = new)]
    pub fn decrypt_key(encrypted_key: String, seed: String) -> Option<Vec<u8>> {
        let xpriv = match ExtendedPrivateKey::from_mnemonic(seed.as_bytes(), None) {
            Ok(v) => v.derive_from_path("m/0/0").unwrap(),
            Err(_) => return None
        };

        let encrypted_key_buf = match base64::decode_config(encrypted_key, base64::STANDARD) {
            Ok(v) => v,
            Err(_) => return None
        };

        let ciphertext = match ECIESCiphertext::from_bytes(&encrypted_key_buf, true) {
            Ok(v) => v,
            Err(_) => return None
        };

        let pubkey = match ciphertext.extract_public_key() {
            Ok(v) => v,
            Err(_) => return None
        };

        let decrypted = match ECIES::decrypt(&ciphertext, &xpriv.get_private_key(), &pubkey) {
            Ok(v) => v,
            Err(_) => return None
        };

        return Some(decrypted)
    }
}
