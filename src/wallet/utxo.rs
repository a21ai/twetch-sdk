use crate::{constants, MetasyncApi, Networks, WhatsOnChainApi};
use anyhow::Result;
use bsv::{P2PKHAddress, PublicKey, Script};

#[derive(Debug)]
pub struct UTXO {
    pub txid: String,
    pub vout: u64,
    pub satoshis: u64,
    pub path: i32,
    pub script: Option<Script>,
    pub contract: Option<String>,
}

impl UTXO {
    pub async fn from_woc(address: &P2PKHAddress, network: &Networks) -> Result<Vec<UTXO>> {
        let whatsonchain = WhatsOnChainApi::new(constants::WHATSONCHAIN_URL.to_string());
        let utxos = whatsonchain
            .utxos(address, network)
            .await?
            .iter()
            .map(|e| UTXO {
                txid: e.tx_hash.clone(),
                vout: e.tx_pos,
                satoshis: e.value,
                path: -1,
                script: None,
                contract: None,
            })
            .collect();

        Ok(utxos)
    }

    pub async fn from_metasync(public_key: &PublicKey, network: &Networks) -> Result<Vec<UTXO>> {
        let metasync = MetasyncApi::new(constants::METASYNC_URL.to_string());

        let utxos = metasync
            .utxos(public_key, network)
            .await?
            .iter()
            .map(|e| UTXO {
                txid: e.txid.clone(),
                vout: e.vout,
                satoshis: e.satoshis.parse::<u64>().unwrap(),
                path: e.path.parse::<i32>().unwrap(),
                script: None,
                contract: None,
            })
            .collect();

        Ok(utxos)
    }
}
