use twetch_sdk::networks;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum Networks {
    BSV,
    TBSV,
}

impl From<Networks> for networks::Networks {
    fn from(v: Networks) -> networks::Networks {
        match v {
            Networks::BSV => networks::Networks::BSV,
            Networks::TBSV => networks::Networks::TBSV,
        }
    }
}
