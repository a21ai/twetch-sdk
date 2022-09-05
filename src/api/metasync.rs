use crate::{Networks, Wallet};
use anyhow::Result;
use bsv::{PublicKey, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Clone)]
pub struct MetasyncUTO {
    pub outpoint: String,
    pub satoshis: String,
    pub contract: String,
    pub token: String,
    pub script: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetasyncUTXO {
    pub txid: String,
    pub vout: u32,
    pub satoshis: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct MetasyncUTXOResponse {
    pub utxos: Vec<MetasyncUTXO>,
}

#[derive(Serialize, Deserialize)]
pub struct MetasyncApi {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentDestinationOutput {
    pub satoshis: u64,
    pub script: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetasyncPaymentDestination {
    pub outputs: Vec<PaymentDestinationOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Broadcast {
    txid: Option<String>,
    error: Option<String>,
}

impl MetasyncApi {
    pub fn new(url: String) -> MetasyncApi {
        MetasyncApi { url }
    }

    pub fn network(network: &Networks) -> String {
        match network {
            Networks::BSV => "BSV".to_string(),
            Networks::TBSV => "TBSV".to_string(),
        }
    }

    pub fn post(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.post(format!("{}{}", self.url, path))
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(format!("{}{}", self.url, path))
    }

    pub async fn uto(&self, outpoint: &String) -> Result<MetasyncUTO> {
        let res = self
            .get(format!("/uto/{}", outpoint))
            .send()
            .await?
            .json::<MetasyncUTO>()
            .await?;

        Ok(res)
    }

    pub async fn utxos(
        &self,
        public_key: &PublicKey,
        network: &Networks,
    ) -> Result<Vec<MetasyncUTXO>> {
        let payload = json!({
            "pubkey": public_key.to_hex()?,
            "amount": 1,
            "newtork": MetasyncApi::network(network)
        });

        let res = self
            .post("/wallet/utxo".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<MetasyncUTXOResponse>()
            .await?;

        Ok(res.utxos)
    }

    pub async fn payment_destination(
        &self,
        paymail: &String,
    ) -> Result<MetasyncPaymentDestination> {
        let payload = json!({
            "satoshis": 0,
        });
        let res = self
            .post(format!("/paymail/p2p-payment-destination/{}", paymail))
            .json(&payload)
            .send()
            .await?
            .json::<MetasyncPaymentDestination>()
            .await?;
        Ok(res)
    }

    pub async fn broadcast(
        &self,
        tx: &Transaction,
        network: &Networks,
        wallet: &Wallet,
    ) -> Result<Broadcast> {
        anyhow::ensure!(
            None != wallet.user_id,
            "MetasyncAPI Error: no user found in wallet"
        );

        let payload = json!({
            "hex": tx.to_hex()?,
            "network": MetasyncApi::network(network),
            "metadata": { "sender": format!("{}@twetch.me", &wallet.user_id.clone().unwrap())  }
        });
        let res = self
            .post("/tx".to_string())
            .json(&payload)
            .send()
            .await?
            .json::<Broadcast>()
            .await?;
        Ok(res)
    }
}
