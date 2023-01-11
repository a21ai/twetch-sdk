use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct RPCClient {
    pub host: String,
    pub port: Option<String>,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct RPCBlockchainInfoResponse {
    pub result: Option<RPCBlockchainInfo>,
    pub id: String,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Default, Debug)]
pub struct RPCBlockchainInfo {
    pub blocks: u32,
    pub chain: String,
    pub headers: u32,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: u32,
    pub verificationprogress: f64,
    pub chainwork: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct RPCError {
    pub code: Option<i32>,
    pub message: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct RPCBlockHashResponse {
    pub result: Option<String>,
    pub id: String,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Default, Debug)]
pub struct RPCBlockResponse {
    pub result: Option<RPCBlock>,
    pub id: String,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Default, Debug)]
pub struct RPCRawBlockResponse {
    pub result: Option<String>,
    pub id: String,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Default, Debug)]
pub struct RPCBlock {
    pub tx: Vec<String>,
}

impl RPCClient {
    pub fn client(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();

        let url = match &self.port {
            Some(port) => format!("{}:{}", self.host, port),
            None => format!("{}", self.host),
        };

        client
            .post(url)
            .basic_auth(&self.user, Some(&self.password))
    }

    pub async fn broadcast_rawtx(&self, rawtx: String) -> Result<bool> {
        let body = json!({
            "method": "sendrawtransaction".to_string(),
            "params": [rawtx],
            "id": "420".to_string(),
        });

        let status = self.client().json(&body).send().await?.status();

        return Ok(status == reqwest::StatusCode::OK);
    }

    pub async fn get_blockchain_info(&self) -> Result<RPCBlockchainInfoResponse> {
        let body = json!({
            "method": "getblockchaininfo".to_string(),
            "id": "420".to_string(),
        });

        let res = self
            .client()
            .json(&body)
            .send()
            .await?
            .json::<RPCBlockchainInfoResponse>()
            .await?;

        return Ok(res);
    }

    pub async fn get_block_header(&self, block_hash: String) -> Result<RPCBlockHashResponse> {
        let body = json!({
            "method": "getblockheader".to_string(),
            "id": "420".to_string(),
            "params": [block_hash, false],
        });

        let res = self
            .client()
            .json(&body)
            .send()
            .await?
            .json::<RPCBlockHashResponse>()
            .await?;

        Ok(res)
    }

    pub async fn get_block_hash(&self, height: i64) -> Result<RPCBlockHashResponse> {
        let body = json!({
            "method": "getblockhash".to_string(),
            "id": "420".to_string(),
            "params": [height],
        });

        let res = self
            .client()
            .json(&body)
            .send()
            .await?
            .json::<RPCBlockHashResponse>()
            .await?;

        return Ok(res);
    }

    pub async fn get_block_raw(&self, block_hash: String) -> Result<RPCRawBlockResponse> {
        let body = json!( {
            "method": "getblock".to_string(),
            "id": "420".to_string(),
            "params": [block_hash, false],
        });

        let res = self
            .client()
            .json(&body)
            .send()
            .await?
            .json::<RPCRawBlockResponse>()
            .await?;

        return Ok(res);
    }
}
