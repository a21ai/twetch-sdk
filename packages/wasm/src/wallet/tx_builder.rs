use crate::{PaymentDestination, Wallet};
use serde_json::json;
use twetch_sdk::wallet;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
pub struct TxBuilder(wallet::TxBuilder);

impl From<wallet::TxBuilder> for TxBuilder {
    fn from(v: wallet::TxBuilder) -> TxBuilder {
        TxBuilder(v)
    }
}

impl From<TxBuilder> for wallet::TxBuilder {
    fn from(v: TxBuilder) -> wallet::TxBuilder {
        v.0
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct BuiltTx(wallet::BuiltTx);

impl From<wallet::BuiltTx> for BuiltTx {
    fn from(v: wallet::BuiltTx) -> BuiltTx {
        BuiltTx(v)
    }
}

impl From<BuiltTx> for wallet::BuiltTx {
    fn from(v: BuiltTx) -> wallet::BuiltTx {
        v.0
    }
}

#[wasm_bindgen]
impl BuiltTx {
    #[wasm_bindgen(getter)]
    pub fn extended_tx(&self) -> Option<String> {
        self.0.tx.to_compact_hex().ok()
    }

    #[wasm_bindgen(getter)]
    pub fn rawtx(&self) -> Option<String> {
        self.0.tx.to_hex().ok()
    }

    #[wasm_bindgen(getter)]
    pub fn txid(&self) -> String {
        self.0.txid.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn total_cost_sats(&self) -> u64 {
        self.0.total_cost_sats
    }

    #[wasm_bindgen(getter)]
    pub fn fee_sats(&self) -> u64 {
        self.0.fee_sats
    }

    #[wasm_bindgen(getter)]
    pub fn num_payment_destinations(&self) -> usize {
        self.0.payment_destinations.len()
    }

    pub fn get_payment_destination(&self, index: usize) -> PaymentDestination {
        self.0.payment_destinations[index].clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn encrypted_hash(&self) -> Option<String> {
        self.0.encrypted_hash.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn nfts(&self) -> JsValue {
        let value = serde_json::to_value(&self.0.nfts).unwrap();
        JsValue::from_serde(&value).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn typed_signing(&self) -> JsValue {
        let value = serde_json::to_value(&self.0.typed_signing).unwrap();
        JsValue::from_serde(&value).unwrap()
    }
}

#[wasm_bindgen]
impl TxBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TxBuilder {
        TxBuilder(wallet::TxBuilder {
            network: wallet::Networks::BSV,
            contract: None,
            outputs: vec![],
            extended_tx: None,
            change_address: None,
            auto_fund: false,
            typed_signing: None,
        })
    }

    pub fn from_json(value: JsValue) -> Result<TxBuilder, JsValue> {
        match value.into_serde::<wallet::TxBuilder>() {
            Ok(v) => Ok(v.into()),
            Err(e) => {
                let payload = json!({
                    "message": format!("{:?}", e),
                });
                Err(JsValue::from_serde(&payload).unwrap())
            }
        }
    }

    pub async fn build(value: JsValue, wallet: Wallet) -> Result<BuiltTx, JsValue> {
        let builder = TxBuilder::from_json(value)?;

        match wallet::TxBuilder::build(&builder.into(), &wallet.into()).await {
            Ok(built) => Ok(built.into()),
            Err(e) => {
                let payload = json!({
                    "message": format!("{:?}", e),
                });
                Err(JsValue::from_serde(&payload).unwrap())
            }
        }
    }
}
