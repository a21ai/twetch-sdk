use crate::{constants, MetasyncApi, Networks, UtxoDetectiveApi, WhatsOnChainApi};
use anyhow::Result;
use bsv::{P2PKHAddress, PublicKey, Script};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UTXO {
    pub txid: String,
    pub vout: u32,
    pub satoshis: u64,
    pub path: i32,
    pub script: Option<Script>,
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
            })
            .collect();

        Ok(utxos)
    }

    pub async fn from_utxo_detective(public_key: &PublicKey, amount: u64) -> Result<Vec<UTXO>> {
        let utxo_detective = UtxoDetectiveApi::new(constants::UTXO_DETECTIVE_URL.to_string());

        let utxos = utxo_detective
            .utxos(public_key, amount)
            .await?
            .iter()
            .map(|e| UTXO {
                txid: e.txid.clone(),
                vout: e.vout,
                satoshis: e.satoshis.parse::<u64>().unwrap(),
                path: e.path.parse::<i32>().unwrap(),
                script: None,
            })
            .collect();

        Ok(utxos)
    }
}
