use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Networks {
    BSV,
    TBSV,
}

impl From<String> for Networks {
    fn from(v: String) -> Networks {
        match v.as_str() {
            "BSV" => Networks::BSV,
            "TBSV" => Networks::TBSV,
            _ => Networks::BSV,
        }
    }
}
