use crate::{PaymentDestination, Wallet};
use anyhow::Result;
use bsv::{P2PKHAddress, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct PolynymApi {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct GetAddress {
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymailCapabilities {
    pub pki: Option<String>,
    pub payment_destination: Option<String>,
    #[serde(rename(deserialize = "a9f510c16bde"))]
    pub verify_pubkey: Option<String>,
    #[serde(rename(deserialize = "f12f968c92d6"))]
    pub public_profile: Option<String>,
    #[serde(rename(deserialize = "5f1323cddf31"))]
    pub p2p_receive_transaction: Option<String>,
    #[serde(rename(deserialize = "2a40af698840"))]
    pub p2p_payment_destination: Option<String>,
    pub sigil: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymailP2PPaymailDestinationOutput {
    pub satoshis: u64,
    pub script: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymailP2PPaymentDestination {
    pub outputs: Vec<PaymailP2PPaymailDestinationOutput>,
    pub reference: String,
}

impl PolynymApi {
    pub fn new(url: String) -> PolynymApi {
        PolynymApi { url }
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(format!("{}{}", self.url, path))
    }

    pub fn post(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.post(format!("{}{}", self.url, path))
    }

    pub async fn get_address(&self, paymail: &String) -> Result<P2PKHAddress> {
        let res = self
            .get(format!("/getAddress/{}", paymail))
            .send()
            .await?
            .json::<GetAddress>()
            .await?;
        Ok(P2PKHAddress::from_string(&res.address)?)
    }

    pub async fn capabilities(&self, paymail: &String) -> Result<PaymailCapabilities> {
        let res = self
            .get(format!("/capabilities/{}", paymail))
            .send()
            .await?
            .json::<PaymailCapabilities>()
            .await?;

        Ok(res)
    }

    pub async fn p2p_payment_destination(
        &self,
        paymail: &String,
        satoshis: u64,
    ) -> Result<PaymailP2PPaymentDestination> {
        let payload = json!({ "satoshis": satoshis });
        let res = self
            .post(format!("/p2p/destination/{}", paymail))
            .json(&payload)
            .send()
            .await?
            .json::<PaymailP2PPaymentDestination>()
            .await?;

        Ok(res)
    }

    pub async fn submit_p2p_payment(
        &self,
        tx: &Transaction,
        payment_destination: &PaymentDestination,
        wallet: &Wallet,
    ) -> Result<()> {
        anyhow::ensure!(
            None != wallet.user_id,
            "PolynymAPI Error: no user found in wallet"
        );

        let payload = json!({ "hex": tx.to_hex()?, "reference": payment_destination.reference, "metadata": {
            "sender": format!("{}@twetch.me", &wallet.user_id.clone().unwrap()),
            "ref": payment_destination.reference,
        } });

        self.post(format!("/p2p/{}", payment_destination.paymail))
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }
}
