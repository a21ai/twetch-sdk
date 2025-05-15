use anyhow::Result;
use bsv::PublicKey;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Clone)]
pub struct UtxoDetectiveUTXO {
    pub txid: String,
    pub vout: u32,
    pub satoshis: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct UtxoDetectivePublicUtxo {
    pub txid: String,
    pub vout: u32,
    pub satoshis: String,
    pub block_height: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UtxoDetectiveUTXOResponse {
    pub utxos: Vec<UtxoDetectiveUTXO>,
}

#[derive(Debug, Deserialize)]
pub struct UtxoDetectiveBalance {
    pub satoshis: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UtxoDetectiveDecodeTxResponse {
    pub outputs: Vec<UtxoDetectiveDecodeTxOutput>,
}

#[derive(Serialize, Deserialize)]
pub struct UtxoDetectiveSpentOutpointResponse {
    pub outpoints: Vec<Option<UtxoDetectiveSpentOutpointValueResponse>>,
}

#[derive(Serialize, Deserialize)]
pub struct UtxoDetectiveSpentOutpointValueResponse {
    pub h: i64,
    pub o: String,
}

#[derive(Serialize, Deserialize)]
pub struct UtxoDetectiveDecodeTxOutput {
    pub alias: Option<String>,
    pub satoshis: u64,
    pub scripthash: String,
}

#[derive(Debug, Deserialize)]
pub struct MempoolCheckResponse {
    pub spent: Vec<bool>,
    pub new_utxos: Vec<UtxoDetectivePublicUtxo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub async fn balance_by_address(&self, address: &String) -> Result<UtxoDetectiveBalance> {
        let res = self
            .get(format!("/balance/{}", address))
            .send()
            .await?
            .json::<UtxoDetectiveBalance>()
            .await?;

        Ok(res)
    }

    pub async fn outpoints(&self, outpoints: Vec<Vec<u8>>) -> Result<Vec<bool>> {
        let payload = json!({
            "outpoints": outpoints.iter().map(|e| hex::encode(e)).collect::<Vec<_>>()
        });

        let res = self
            .post(format!("/outpoints"))
            .json(&payload)
            .send()
            .await?
            .json::<Vec<bool>>()
            .await?;

        Ok(res)
    }

    pub async fn mempool_spends(&self, outpoints: Vec<Vec<u8>>) -> Result<Vec<bool>> {
        let payload = json!({
            "outpoints": outpoints.iter().map(|e| hex::encode(e)).collect::<Vec<_>>()
        });

        let res = self
            .post(format!("/mempool/spends"))
            .json(&payload)
            .send()
            .await?
            .json::<Vec<bool>>()
            .await?;

        Ok(res)
    }

    pub async fn mempool_check(
        &self,
        outpoints: Vec<Vec<u8>>,
        address: Option<String>,
    ) -> Result<(Vec<bool>, Vec<UtxoDetectivePublicUtxo>)> {
        let mut payload = json!({});

        if !outpoints.is_empty() {
            payload["outpoints"] =
                json!(outpoints.iter().map(|e| hex::encode(e)).collect::<Vec<_>>());
        }

        if let Some(addr) = address {
            payload["address"] = json!(addr);
        }

        let response = self
            .post(format!("/mempool"))
            .json(&payload)
            .send()
            .await?
            .json::<MempoolCheckResponse>()
            .await?;

        Ok((response.spent, response.new_utxos))
    }

    pub async fn spends_by_outpoint(
        &self,
        outpoints: Vec<Vec<u8>>,
    ) -> Result<Vec<Option<(Vec<u8>, i64)>>> {
        let payload = json!({
            "outpoints": outpoints.iter().map(|e| hex::encode(e)).collect::<Vec<_>>()
        });

        let res = self
            .post(format!("/spends/values"))
            .json(&payload)
            .send()
            .await?
            .json::<UtxoDetectiveSpentOutpointResponse>()
            .await?;

        let response = res
            .outpoints
            .iter()
            .map(|e| match e {
                Some(v) => Some((hex::decode(&v.o).unwrap(), v.h)),
                None => None,
            })
            .collect::<Vec<_>>();

        Ok(response)
    }

    pub async fn utxos_by_address(&self, address: &String) -> Result<Vec<UtxoDetectivePublicUtxo>> {
        let res = self
            .get(format!("/utxos/{}", address))
            .send()
            .await?
            .json::<Vec<UtxoDetectivePublicUtxo>>()
            .await?;

        Ok(res)
    }

    pub async fn decode_tx(&self, rawtx: String) -> Result<UtxoDetectiveDecodeTxResponse> {
        let payload = json!({
            "hex": rawtx,
        });

        let res = self
            .post("/sync/decode-tx".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<UtxoDetectiveDecodeTxResponse>()
            .await?;

        Ok(res)
    }

    pub async fn sync_tx(&self, rawtx: String) -> Result<Value> {
        let payload = json!({
            "hex": rawtx,
        });

        let res = self
            .post("/sync/tx".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
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
