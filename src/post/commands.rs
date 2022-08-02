use regex::Regex;

pub enum Currency {
    BSV,
    USD,
}

pub struct PayCommand {
    pub amount: f64,
    pub currency: Currency,
}

impl PayCommand {
    pub fn from_string(description: &String) -> Option<PayCommand> {
        let pay_any_regex = Regex::new(r"(/[pP][aA][yY])\s+((@\d+\s+)|([a-zA-Z-_\d]+@[a-zA-Z-_\d.]+[a-zA-Z\d]\s+)|([1][a-km-zA-HJ-NP-Z\d]{25,34}\s+)|([$][a-zA-Z-_.]{4,50}\s+)|([1][a-zA-Z\d]+\s+))+(((((\d{1,8})?(.\d{1,8}))|((\d{1,8})(.\d{1,8})?))\s*([bB][sS][vV]))|([$][\d]+(.[\d]+)?))").unwrap();
        let pay_any_currency = Regex::new(r"(?P<bsv>(((\d{1,8})?\.\d{1,8})|(\d{1,8}(\.\d{1,8})?))\s*[bB][sS][vV])|(?P<usd>[$][\d]*(\.[\d]+)?)").unwrap();

        let mut currency = Currency::USD;
        let mut amount = 0f64;

        let pay_match = match pay_any_regex.find(&description) {
            None => return None,
            Some(p) => p,
        };

        let pay_match_str = pay_match.as_str();

        println!("pay_match_str {:?}", pay_match_str);

        let pay_currency_match = pay_any_currency
            .find_iter(pay_match_str)
            .last()
            .unwrap()
            .as_str();
        let captures = pay_any_currency.captures(pay_currency_match).unwrap();

        match captures.name("usd") {
            None => (),
            Some(value) => {
                let mut chars = value.as_str().chars();
                chars.next(); // pop the leading $
                amount = chars.as_str().to_string().parse::<f64>().unwrap();
            }
        }

        match captures.name("bsv") {
            None => (),
            Some(_value) => {
                currency = Currency::BSV;
                amount = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
            }
        }

        return Some(PayCommand { amount, currency });
    }

    pub fn get_amount_usd(&self, exchange_rate: &f64) -> f64 {
        match &self.currency {
            Currency::BSV => {
                return self.amount * exchange_rate;
            }
            Currency::USD => {
                return self.amount;
            }
        }
    }

    pub fn get_amount_bsv(&self, exchange_rate: &f64) -> f64 {
        match &self.currency {
            Currency::BSV => {
                return self.amount;
            }
            Currency::USD => {
                return self.amount / exchange_rate;
            }
        }
    }
}
