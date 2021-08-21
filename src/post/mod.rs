use wasm_bindgen::{prelude::*, JsValue, throw_str};
use serde::*;
use regex::Regex;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {}

impl Post {
}

#[wasm_bindgen]
impl Post {
    #[wasm_bindgen(js_name = estimateUsd)]
    pub fn estimate_usd(description: String, exchange_rate: f32) -> f32 {
        if (description.chars().count() <= 0) {
            return 0.0;
        }

        let mut sum = 0.02;

        let PAY_ANY = Regex::new(r"(\/[pP][aA][yY])\s+((\@\d+\s+)|([a-zA-Z\-\_\d]+@[a-zA-Z\-\_\d\.]+[a-zA-Z\d]\s+)|([1][a-km-zA-HJ-NP-Z\d]{25,34}\s+)|([$][a-zA-Z\d-_.]{4,50}\s+)|([1][a-zA-Z\d]+\s+))+(((((\d{1,8})?(\.\d{1,8}))|((\d{1,8})(\.\d{1,8})?))\s*([bB][sS][vV]))|([$][\d]+(\.[\d]+)?))").unwrap();

        let pay_match = PAY_ANY.find(&description).unwrap().as_str();

        if (pay_match.chars().count() > 0) {
            sum += 1.0;
            //let PAY_ANY_CURRENCY = Regex::new(r"/((((\d{1,8})?\.\d{1,8})|(\d{1,8}(\.\d{1,8})?))\s*[bB][sS][vV])|([$][\d]*(\.[\d]+)?)/g").unwrap();
            //let PAY_ANY_CURRENCY_BSV = Regex::new(r"/(((\d{1,8})?\.\d{1,8})|(\d{1,8}(\.\d{1,8})?))\s*[bB][sS][vV]/g").unwrap();
            //let PAY_ANY_CURRENCY_USD = Regex::new(r"/([$][\d]*(\.[\d]+)?)/g").unwrap();

            //for (cap in PAY_ANY_CURRENCY.capture_iter(text) {
            //}
        }

        return sum;
    }
}
