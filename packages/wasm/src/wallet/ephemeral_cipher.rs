use twetch_sdk::wallet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EphemeralCipher(wallet::EphemeralCipher);

impl From<wallet::EphemeralCipher> for EphemeralCipher {
    fn from(v: wallet::EphemeralCipher) -> EphemeralCipher {
        EphemeralCipher(v)
    }
}

impl From<EphemeralCipher> for wallet::EphemeralCipher {
    fn from(v: EphemeralCipher) -> wallet::EphemeralCipher {
        v.0
    }
}

#[wasm_bindgen]
impl EphemeralCipher {
    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> Vec<u8> {
        self.0.hash.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn cipher_text(&self) -> Vec<u8> {
        self.0.cipher_text.clone()
    }
}
