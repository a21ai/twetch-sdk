use crate::api::Api;
use anyhow::Result;
use bsv_wasm::{AESAlgorithms, AES};
use js_sys::decode_uri_component;
use serde::*;
use serde_json::json;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    plaintext: String,
}

impl Message {
    pub fn encrypt(key: &[u8], plaintext: String) -> Option<String> {
        let iv = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let result = match AES::encrypt(key, iv, plaintext.as_bytes(), AESAlgorithms::AES128_CTR) {
            Ok(v) => v,
            Err(_) => return None,
        };

        Some(hex::encode(result))
    }

    pub async fn create(
        key: String,
        conversation_id: String,
        user_id: String,
        description: String,
        token: String,
    ) -> Result<Message> {
        println!(
            "{:?} {:?} {:?} {:?} {:?}",
            key, conversation_id, user_id, description, token
        );

        let encrypted = Message::encrypt(&hex::decode(key).unwrap(), description.clone());

        let payload = json!({
            "payload": {
                "description": encrypted,
                "userId": user_id,
                "conversationId": conversation_id
            }
        });

        println!("payload {:?}", payload);

        let api = Api { token };
        let res = api.create_message(payload).await?;

        println!("message create response {:?}", res);

        Ok(Message {
            plaintext: description,
        })
    }
}

#[wasm_bindgen]
impl Message {
    #[wasm_bindgen(js_name = encrypt)]
    pub fn encrypt_wasm(key: &[u8], plaintext: String) -> Option<String> {
        Message::encrypt(key, plaintext)
    }

    #[wasm_bindgen(js_name = decrypt)]
    pub fn decrypt(key: &[u8], description: &[u8]) -> Option<Message> {
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

        return Some(Message {
            plaintext: decoded_utf8.into(),
        });
    }

    #[wasm_bindgen(js_name = plaintext)]
    pub fn plaintext(&self) -> String {
        return self.plaintext.clone();
    }
}
