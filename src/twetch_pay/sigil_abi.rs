use crate::{constants, Outpoint, TxlogApi};
use anyhow::Result;
use bsv::{P2PKHAddress, Script};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sigil_types::{TXID, UTO};

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

pub async fn get_uto(outpoint: String, contract: String) -> Result<UTO> {
    let (txid, vout) = Outpoint::decode(outpoint.to_string())?;

    let txlog_api = TxlogApi::new(constants::urls::TXLOG_URL.to_string());

    let satoshis_in = txlog_api.satoshis(&txid, vout).await?;
    let token_script = txlog_api.script(&txid, vout).await?;

    let token_meta = match token_script.to_asm_string().rsplit_once(" ") {
        Some(v) => v.1.to_string(),
        None => anyhow::bail!("invalid token meta"),
    };

    let contract_bytes = format!("{:?}", hex::decode(contract)?);
    let contract_txid: TXID = TXID(serde_json::from_str(&contract_bytes)?);

    let bits = token_script.to_script_bits();

    let from_script = Script::from_script_bits(vec![
        bits[3].clone(),
        bits[4].clone(),
        bits[5].clone(),
        bits[6].clone(),
        bits[7].clone(),
    ]);

    let uto = UTO {
        outpoint: hex::decode(outpoint)?.try_into()?,
        satoshis: satoshis_in,
        contract: contract_txid,
        token: hex::decode(token_meta)?,
        script: from_script.clone(),
        value: None,
    };

    Ok(uto)
}
