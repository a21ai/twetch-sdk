use crate::PayParams;
use anyhow::Result;
use bsv::Transaction;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct Api {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payee {
    pub amount: Value,
    pub sats: Option<u64>,
    pub to: String,
    pub types: Option<Vec<String>>,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesResponse {
    pub errors: Vec<String>,
    pub invoice: String,
    pub payees: Vec<Payee>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishResponse {
    pub errors: Option<Vec<String>>,
    pub broadcasted: Option<bool>,
    pub publish: Option<bool>,
    pub publish_params: Option<PublishParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishParams {
    pub token: Option<String>,
}

impl Api {
    pub fn new(url: String, token: String) -> Api {
        Api { url, token }
    }

    pub fn post(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(format!("{}{}", self.url, path))
            .header("Authorization", format!("Bearer {}", self.token))
    }

    pub async fn payees(&self, action: &String, args: &Vec<String>) -> Result<PayeesResponse> {
        let payload = json!({
            "args": args,
            "action": action,
            "client_identifier": "1325c30a-7eb3-4169-a6f4-330eeeb8ca49",
            "payload": {
                "resolveChange": true
            }
        });

        let res = self
            .post("/v1/payees".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<PayeesResponse>()
            .await?;

        Ok(res)
    }

    pub async fn publish(
        &self,
        action: &String,
        tx: &Transaction,
        pay_params: Option<PayParams>,
    ) -> Result<PublishResponse> {
        let payload = json!({
            "broadcast": true,
            "action": action,
            "signed_raw_tx": tx.to_hex()?,
            "payParams": pay_params
        });

        let res = self
            .post("/v1/publish".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<PublishResponse>()
            .await?;

        Ok(res)
    }
}
