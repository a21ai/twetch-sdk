pub mod commands;

use twetch_sdk::post;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Post(post::Post);

#[wasm_bindgen]
impl Post {
    #[wasm_bindgen(js_name = fromDescription)]
    pub fn from_description(description: String) -> Post {
        Post(post::Post::from_description(description))
    }

    #[wasm_bindgen(js_name = estimateUsd)]
    pub fn estimate_usd(&self, exchange_rate: f64) -> f64 {
        post::Post::estimate_usd(&self.0, exchange_rate)
    }

    #[wasm_bindgen(js_name = getPayCommand)]
    pub fn get_pay_command(&self, exchange_rate: f64) -> Option<String> {
        post::Post::get_pay_command(&self.0, exchange_rate)
    }
}
