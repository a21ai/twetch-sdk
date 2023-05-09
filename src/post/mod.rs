pub mod commands;
pub mod mentions;

use crate::post::{commands::PayCommand, mentions::Mentions};

pub struct Post {
    description: String,
}

impl Post {
    pub fn from_description(description: String) -> Post {
        return Post { description };
    }

    pub fn estimate_usd(&self, exchange_rate: f64) -> f64 {
        if self.description.chars().count() <= 0 {
            return 0.00f64;
        }

        let mut sum = 0.001f64;

        sum += match PayCommand::from_string(&self.description) {
            None => 0f64,
            Some(pay_command) => pay_command.get_amount_usd(&exchange_rate),
        };

        sum += match Mentions::from_string(&self.description) {
            None => 0f64,
            Some(mentions) => mentions.estimate_usd,
        };

        return format!("{:.1$}", sum, 3).parse::<f64>().unwrap();
    }

    pub fn get_pay_command(&self, exchange_rate: f64) -> Option<String> {
        let pay_command = match PayCommand::from_string(&self.description) {
            None => return None,
            Some(p) => p,
        };

        let _bsv = pay_command.get_amount_bsv(&exchange_rate);
        let _usd = pay_command.get_amount_usd(&exchange_rate);

        return Some("".to_string());
    }
}
