use twetch_sdk::wallet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PaymentDestination(wallet::PaymentDestination);

impl From<wallet::PaymentDestination> for PaymentDestination {
    fn from(v: wallet::PaymentDestination) -> PaymentDestination {
        PaymentDestination(v)
    }
}

impl From<PaymentDestination> for wallet::PaymentDestination {
    fn from(v: PaymentDestination) -> wallet::PaymentDestination {
        v.0
    }
}

#[wasm_bindgen]
impl PaymentDestination {
    #[wasm_bindgen(getter)]
    pub fn paymail(&self) -> String {
        self.0.paymail.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn reference(&self) -> String {
        self.0.reference.clone()
    }
}
