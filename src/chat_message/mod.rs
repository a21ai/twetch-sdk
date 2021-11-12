use bsv_wasm::{AESAlgorithms, AES};
use js_sys::decode_uri_component;
use serde::*;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    plaintext: String,
}

#[wasm_bindgen]
impl ChatMessage {
    #[wasm_bindgen(js_name = new)]
    pub fn new(key: &[u8], description: &[u8]) -> Option<ChatMessage> {
        let iv = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let result = match AES::decrypt(key, iv, description, AESAlgorithms::AES128_CTR) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let utf8 = match str::from_utf8(&result) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let decoded_utf8 = match decode_uri_component(&utf8) {
            Ok(v) => v,
            Err(_) => return None,
        };

        return Some(ChatMessage {
            plaintext: decoded_utf8.into(),
        });
    }

    #[wasm_bindgen(js_name = plaintext)]
    pub fn plaintext(&self) -> String {
        return self.plaintext.clone();
    }
}
