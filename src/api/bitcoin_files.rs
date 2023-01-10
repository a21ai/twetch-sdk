use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct BitcoinFilesApi {
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContractInit {
    pub version: String,
    pub name: Option<String>,
    pub creator_address: String,
    pub sats_out: Option<u64>,
    pub royalty_percentage: Option<[u32; 2]>,
    pub mint_outpoint: Option<String>,
    pub mint_outpoint_sats: Option<u64>,
    pub total_supply: Option<u64>,
}

impl BitcoinFilesApi {
    pub fn new(url: String) -> BitcoinFilesApi {
        BitcoinFilesApi { url }
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(format!("{}{}", self.url, path))
    }

    pub async fn contract(&self, txid: &String) -> Result<ContractInit> {
        let res = self
            .get(format!("/{}", txid))
            .send()
            .await?
            .json::<ContractInit>()
            .await?;

        Ok(res)
    }
}
