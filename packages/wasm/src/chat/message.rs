use twetch_sdk::chat::message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ChatMessage(message::Message);

#[wasm_bindgen]
impl ChatMessage {
    pub fn encrypt(key: &[u8], plaintext: String) -> Option<String> {
        match message::Message::encrypt(key, plaintext) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn decrypt(key: &[u8], description: &[u8]) -> Option<ChatMessage> {
        match message::Message::decrypt(key, description) {
            Some(v) => Some(ChatMessage(v)),
            None => None,
        }
    }

    pub fn plaintext(&self) -> String {
        return self.0.plaintext.clone();
    }
}
