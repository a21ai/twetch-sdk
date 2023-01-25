use crate::api::rpc::BroadcastResponse;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct MapiApi {
    pub url: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastMapiResponse {
    pub payload: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastMapiResponsePayload {
    pub txid: String,
    pub returnResult: String,
    pub resultDescription: String,
}

const VALID_ERRORS: [&str; 5] = [
    "ERROR: 257: txn-already-known",             // mapi
    "257: txn-already-known",                    // node
    "ERROR: Transaction already in the mempool", // mapi
    "Transaction already known",                 // mapi
    "Transaction already in the mempool",        // node and mapi
];

impl MapiApi {
    pub fn new(url: String, token: String) -> MapiApi {
        MapiApi { url, token }
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client
            .get(format!("{}{}", self.url, path))
            .header("Authorization", format!("Bearer {}", self.token))
    }

    pub fn post(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(format!("{}{}", self.url, path))
            .header("Authorization", format!("Bearer {}", self.token))
    }

    pub async fn broadcast_rawtx(&self, rawtx: &Vec<u8>) -> Result<BroadcastResponse> {
        let res = self
            .post("/tx".to_string())
            .header("Content-Type", "application/octet-stream")
            .body(rawtx.clone())
            .send()
            .await?
            .json::<BroadcastMapiResponse>()
            .await?;

        let response_payload = match &res.payload {
            Some(v) => v,
            None => {
                return Ok(BroadcastResponse {
                    success: false,
                    response: serde_json::to_value(res)?,
                })
            }
        };

        let payload: BroadcastMapiResponsePayload = match serde_json::from_str(response_payload) {
            Ok(v) => v,
            Err(_) => {
                return Ok(BroadcastResponse {
                    success: false,
                    response: serde_json::to_value(res)?,
                })
            }
        };

        if payload.returnResult == "success".to_string() {
            return Ok(BroadcastResponse {
                success: true,
                response: serde_json::to_value(res)?,
            });
        }

        let is_valid_error = match VALID_ERRORS
            .iter()
            .find(|e| e.to_string() == payload.resultDescription)
        {
            Some(_) => true,
            None => false,
        };

        if payload.returnResult == "failure" && is_valid_error {
            return Ok(BroadcastResponse {
                success: true,
                response: serde_json::to_value(res)?,
            });
        }

        return Ok(BroadcastResponse {
            success: false,
            response: serde_json::to_value(res)?,
        });
    }
}
