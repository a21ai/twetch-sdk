use crate::commands::PayCommand;
use crate::mentions::Mentions;
use serde::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    description: String,
}

#[wasm_bindgen]
impl Post {
    #[wasm_bindgen(js_name = fromDescription)]
    pub fn from_description(description: String) -> Post {
        return Post { description };
    }

    #[wasm_bindgen(js_name = estimateUsd)]
    pub fn estimate_usd(&self, exchange_rate: f64) -> f64 {
        if self.description.chars().count() <= 0 {
            return 0.00f64;
        }

        let mut sum = 0.02f64;

        sum += match PayCommand::from_string(&self.description) {
            None => 0f64,
            Some(pay_command) => pay_command.get_amount_usd(&exchange_rate),
        };

        sum += match Mentions::from_string(&self.description) {
            None => 0f64,
            Some(mentions) => mentions.estimate_usd,
        };

        return format!("{:.1$}", sum, 2).parse::<f64>().unwrap();
    }

    #[wasm_bindgen(js_name = getPayCommand)]
    pub fn get_pay_command(&self, exchange_rate: f64) -> Option<String> {
        let pay_command = match PayCommand::from_string(&self.description) {
            None => return None,
            Some(p) => p,
        };

        let bsv = pay_command.get_amount_bsv(&exchange_rate);
        let usd = pay_command.get_amount_usd(&exchange_rate);

        return Some("".to_string());
    }
}
