use crate::{constants, MetasyncApi};
use anyhow::Result;
use bsv::{P2PKHAddress, Script};
use hex::FromHex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sigil_types::{Outpoint, TXID, UTO};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SigilABI {
    pub contract: String,
    pub method: SigilABIMethods,
    pub params: Vec<SigilABIParam>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SigilABIParam {
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SigilABIMethods {
    Escrow,
    Mint,
    Transfer,
    Vax,
    Purchase,
}

impl From<SigilABIParam> for String {
    fn from(v: SigilABIParam) -> String {
        v.value.as_str().unwrap().to_string()
    }
}

impl From<SigilABIParam> for Script {
    fn from(v: SigilABIParam) -> Script {
        let address: String = v.into();
        P2PKHAddress::from_string(&address)
            .unwrap()
            .get_locking_script()
            .unwrap()
    }
}

impl From<SigilABIParam> for u64 {
    fn from(v: SigilABIParam) -> u64 {
        v.value.as_u64().unwrap()
    }
}

pub async fn get_uto(outpoint: String) -> Result<UTO> {
    let metasync_api = MetasyncApi::new(constants::urls::METASYNC_URL.to_string());
    let metasync_uto = metasync_api.uto(&outpoint).await?;

    let uto = UTO {
        outpoint: Outpoint::from_hex(metasync_uto.outpoint)?,
        satoshis: metasync_uto.satoshis.parse::<u64>()?,
        contract: TXID::from_hex(metasync_uto.contract)?,
        token: hex::decode(metasync_uto.token)?,
        script: Script::from_hex(&metasync_uto.script)?,
        value: None,
    };

    Ok(uto)
}
