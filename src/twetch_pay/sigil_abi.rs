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
    Slurp,
}

impl From<SigilABIParam> for TXID {
    fn from(v: SigilABIParam) -> TXID {
        TXID::from_hex(v.value.as_str().unwrap().to_string()).unwrap()
    }
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

impl From<SigilABIParam> for Outpoint {
    fn from(v: SigilABIParam) -> Outpoint {
        let outpoint: String = v.into();
        Outpoint::from_hex(outpoint).unwrap()
    }
}

impl From<SigilABIParam> for Vec<UTO> {
    fn from(v: SigilABIParam) -> Vec<UTO> {
        let values: Vec<Value> = serde_json::from_value(v.value.clone()).unwrap();

        let utos: Vec<UTO> = values
            .iter()
            .map(|e| UTO {
                outpoint: Outpoint([0u8; 36]),
                satoshis: 2180,
                contract: TXID::from_hex(e.get("contract").unwrap().as_str().unwrap()).unwrap(),
                token: hex::decode(e.get("token").unwrap().as_str().unwrap()).unwrap(),
                script: Script::from_hex(e.get("script").unwrap().as_str().unwrap()).unwrap(),
                value: None,
            })
            .collect();

        utos
    }
}

pub fn get_mint_utos(v: SigilABIParam, contract: String) -> Vec<UTO> {
    let values: Vec<Value> = serde_json::from_str(v.value.as_str().unwrap()).unwrap();

    let utos: Vec<UTO> = values
        .iter()
        .map(|e| UTO {
            outpoint: Outpoint([0u8; 36]),
            satoshis: 2180,
            contract: TXID::from_hex(contract.clone()).unwrap(),
            token: hex::decode(e.get("token").unwrap().as_str().unwrap()).unwrap(),
            script: Script::from_hex(e.get("script").unwrap().as_str().unwrap()).unwrap(),
            value: None,
        })
        .collect();

    utos
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
