use anyhow::Result;
use bsv::PublicKey;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Clone)]
pub struct UtxoDetectiveUTXO {
    pub txid: String,
    pub vout: u32,
    pub satoshis: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct UtxoDetectiveUTXOResponse {
    pub utxos: Vec<UtxoDetectiveUTXO>,
}

#[derive(Serialize, Deserialize)]
pub struct UtxoDetectiveApi {
    url: String,
}

impl UtxoDetectiveApi {
    pub fn new(url: String) -> UtxoDetectiveApi {
        UtxoDetectiveApi { url }
    }

    pub fn post(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.post(format!("{}{}", self.url, path))
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(format!("{}{}", self.url, path))
    }

    pub async fn utxos(
        &self,
        public_key: &PublicKey,
        amount: u64,
    ) -> Result<Vec<UtxoDetectiveUTXO>> {
        let payload = json!({
            "pubkey": public_key.to_hex()?,
            "amount": amount,
        });

        let res = self
            .post("/metasync/wallet/utxos".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<UtxoDetectiveUTXOResponse>()
            .await?;

        Ok(res.utxos)
    }
}
