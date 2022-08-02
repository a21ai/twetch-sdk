use twetch_sdk::chat::conversation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Conversation)]
pub struct Conversation(conversation::Conversation);

#[wasm_bindgen]
impl Conversation {
    #[wasm_bindgen(js_name = generateKey)]
    pub fn generate_key() -> String {
        conversation::Conversation::generate_key()
    }

    #[wasm_bindgen(js_name = encrypt)]
    pub fn encrypt_key(key: String, pubkey: String) -> Option<String> {
        conversation::Conversation::encrypt_key(key, pubkey)
    }

    #[wasm_bindgen(js_name = decrypt)]
    pub fn decrypt_key(encrypted_key: String, seed: String) -> Option<Vec<u8>> {
        conversation::Conversation::decrypt_key(encrypted_key, seed)
    }
}
