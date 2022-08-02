use twetch_sdk::post::commands;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PayCommand(commands::PayCommand);

#[wasm_bindgen]
impl PayCommand {
    pub fn from_string(description: String) -> Option<PayCommand> {
        match commands::PayCommand::from_string(&description) {
            Some(v) => Some(PayCommand(v)),
            None => None,
        }
    }

    pub fn get_amount_usd(&self, exchange_rate: f64) -> f64 {
        commands::PayCommand::get_amount_usd(&self.0, &exchange_rate)
    }

    pub fn get_amount_bsv(&self, exchange_rate: f64) -> f64 {
        commands::PayCommand::get_amount_bsv(&self.0, &exchange_rate)
    }
}
