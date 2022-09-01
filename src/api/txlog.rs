use anyhow::Result;
use bsv::Script;

pub struct TxlogApi {
    url: String,
}

impl TxlogApi {
    pub fn new(url: String) -> TxlogApi {
        TxlogApi { url }
    }

    pub fn get(&self, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(format!("{}{}", self.url, path))
    }

    pub async fn script(&self, txid: &String, vout: u32) -> Result<Script> {
        let res = self
            .get(format!("/tx/{}/{}/script", txid, vout))
            .send()
            .await?
            .text()
            .await?;

        Ok(Script::from_hex(&res)?)
    }

    pub async fn satoshis(&self, txid: &String, vout: u32) -> Result<u64> {
        let res = self
            .get(format!("/tx/{}/{}/satoshis", txid, vout))
            .send()
            .await?
            .text()
            .await?
            .parse::<u64>()?;

        Ok(res)
    }
}
