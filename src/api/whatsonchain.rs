use crate::Networks;
use anyhow::Result;
use bsv::P2PKHAddress;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WhatsOnChainUTXO {
    pub tx_hash: String,
    pub tx_pos: u32,
    pub value: u64,
}

pub struct WhatsOnChainApi {
    url: String,
}

impl WhatsOnChainApi {
    pub fn new(url: String) -> WhatsOnChainApi {
        WhatsOnChainApi { url }
    }

    pub fn network(network: &Networks) -> String {
        match network {
            Networks::BSV => "main".to_string(),
            Networks::TBSV => "test".to_string(),
        }
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(format!("{}{}", self.url, path))
    }

    pub async fn utxos(
        &self,
        address: &P2PKHAddress,
        network: &Networks,
    ) -> Result<Vec<WhatsOnChainUTXO>> {
        let scripthash = address.get_locking_script()?.to_scripthash_hex();

        let res = self
            .get(format!(
                "/v1/bsv/{}/script/{}/unspent",
                WhatsOnChainApi::network(network),
                scripthash
            ))
            .send()
            .await?
            .json::<Vec<WhatsOnChainUTXO>>()
            .await?;

        Ok(res)
    }
}
