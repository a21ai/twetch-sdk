use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct MapiApi {
    pub url: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastMapiResponse {
    payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastMapiResponsePayload {
    txid: String,
    returnResult: String,
    resultDescription: String,
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

    pub async fn broadcast_rawtx(&self, rawtx: Vec<u8>) -> Result<bool> {
        let res = self
            .post("/tx".to_string())
            .body(rawtx)
            .send()
            .await?
            .json::<BroadcastMapiResponse>()
            .await?;

        println!("{:#?}", res);

        let response_payload = match &res.payload {
            Some(v) => v,
            None => return Ok(false),
        };

        let payload: BroadcastMapiResponsePayload = match serde_json::from_str(response_payload) {
            Ok(v) => v,
            Err(_) => return Ok(false),
        };

        if payload.returnResult == "success".to_string() {
            return Ok(true);
        }

        let is_valid_error = match VALID_ERRORS
            .iter()
            .find(|e| e.to_string() == payload.resultDescription)
        {
            Some(_) => true,
            None => false,
        };

        if payload.returnResult == "failure" && is_valid_error {
            return Ok(true);
        }

        Ok(false)
    }
}
