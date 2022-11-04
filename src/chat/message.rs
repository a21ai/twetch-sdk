use crate::{constants, GraphqlApi};
use anyhow::Result;
use bsv::{AESAlgorithms, AES};
use js_sys::decode_uri_component;
use serde_json::json;
use std::str;

pub struct Message {
    pub plaintext: String,
}

impl Message {
    pub fn encrypt(key: &[u8], plaintext: String) -> Result<String> {
        let iv = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let result = AES::encrypt(key, iv, plaintext.as_bytes(), AESAlgorithms::AES128_CTR)?;
        Ok(hex::encode(result))
    }

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

    pub async fn create(
        key: String,
        conversation_id: String,
        user_id: String,
        description: String,
        token: String,
    ) -> Result<Message> {
        let encrypted = Message::encrypt(&hex::decode(key).unwrap(), description.clone())?;

        let payload = json!({
            "payload": {
                "description": encrypted,
                "userId": user_id,
                "conversationId": conversation_id
            }
        });

        let api = GraphqlApi::new(constants::GATEWAY_URL.to_string(), token);
        api.create_message(payload).await?;

        Ok(Message {
            plaintext: description,
        })
    }
}

impl Message {
    pub fn encrypt_wasm(key: &[u8], plaintext: String) -> Result<String> {
        Message::encrypt(key, plaintext)
    }

    pub fn plaintext(&self) -> String {
        return self.plaintext.clone();
    }
}
