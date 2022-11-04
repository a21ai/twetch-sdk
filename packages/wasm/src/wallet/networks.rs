use serde::{Deserialize, Serialize};
use twetch_sdk::networks;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Networks {
    #[serde(rename(deserialize = "TBSV", serialize = "TBSV"))]
    TBSV,
    #[serde(rename(deserialize = "BSV", serialize = "BSV"))]
    BSV,
}

impl From<Networks> for networks::Networks {
    fn from(v: Networks) -> networks::Networks {
        match v {
            Networks::BSV => networks::Networks::BSV,
            Networks::TBSV => networks::Networks::TBSV,
        }
    }
}
