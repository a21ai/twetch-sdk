use crate::{BuiltTx, Wallet};
use serde_json::json;
use twetch_sdk::{
    twetch_pay, PublishParams as RPublishParams, TwetchPayActionType as RTwetchPayActionType,
};

use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
pub struct TwetchPay(twetch_pay::TwetchPay);

impl From<twetch_pay::TwetchPay> for TwetchPay {
    fn from(v: twetch_pay::TwetchPay) -> TwetchPay {
        TwetchPay(v)
    }
}

impl From<TwetchPay> for twetch_pay::TwetchPay {
    fn from(v: TwetchPay) -> twetch_pay::TwetchPay {
        v.0
    }
}

#[wasm_bindgen]
pub struct TwetchPayCall(twetch_pay::TwetchPayCall);

impl From<twetch_pay::TwetchPayCall> for TwetchPayCall {
    fn from(v: twetch_pay::TwetchPayCall) -> TwetchPayCall {
        TwetchPayCall(v)
    }
}

impl From<TwetchPayCall> for twetch_pay::TwetchPayCall {
    fn from(v: TwetchPayCall) -> twetch_pay::TwetchPayCall {
        v.0
    }
}

#[wasm_bindgen]
pub struct Payee(twetch_sdk::Payee);

impl From<twetch_sdk::Payee> for Payee {
    fn from(v: twetch_sdk::Payee) -> Payee {
        Payee(v)
    }
}

impl From<Payee> for twetch_sdk::Payee {
    fn from(v: Payee) -> twetch_sdk::Payee {
        v.0
    }
}

#[wasm_bindgen]
pub struct TwetchPayAction(twetch_pay::TwetchPayAction);

impl From<twetch_pay::TwetchPayAction> for TwetchPayAction {
    fn from(v: twetch_pay::TwetchPayAction) -> TwetchPayAction {
        TwetchPayAction(v)
    }
}

impl From<TwetchPayAction> for twetch_pay::TwetchPayAction {
    fn from(v: TwetchPayAction) -> twetch_pay::TwetchPayAction {
        v.0
    }
}

#[wasm_bindgen]
pub enum TwetchPayActionType {
    Twetch,
    Sigil,
}

impl From<TwetchPayActionType> for RTwetchPayActionType {
    fn from(v: TwetchPayActionType) -> RTwetchPayActionType {
        match v {
            TwetchPayActionType::Twetch => RTwetchPayActionType::Twetch,
            TwetchPayActionType::Sigil => RTwetchPayActionType::Sigil,
        }
    }
}

impl From<RTwetchPayActionType> for TwetchPayActionType {
    fn from(v: RTwetchPayActionType) -> TwetchPayActionType {
        match v {
            RTwetchPayActionType::Twetch => TwetchPayActionType::Twetch,
            RTwetchPayActionType::Sigil => TwetchPayActionType::Sigil,
        }
    }
}

#[wasm_bindgen]
impl Payee {
    pub fn get_type(&self, index: usize) -> String {
        self.0.types.clone().unwrap()[index].clone()
    }

    #[wasm_bindgen(getter)]
    pub fn num_types(&self) -> usize {
        match self.0.types.clone() {
            Some(v) => v.len(),
            None => 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn user_id(&self) -> String {
        self.0.user_id.clone().unwrap()
    }
}

#[wasm_bindgen]
impl TwetchPayAction {
    #[wasm_bindgen(getter)]
    pub fn built_tx(&self) -> BuiltTx {
        self.0.built_tx.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn is_troll_toll(&self) -> Option<bool> {
        self.0.is_troll_toll
    }

    #[wasm_bindgen(getter)]
    pub fn call(&self) -> TwetchPayCall {
        self.0.call.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn num_payees(&self) -> usize {
        self.0.payees.len()
    }

    pub fn get_payee(&self, index: usize) -> Payee {
        self.0.payees[index].clone().into()
    }
}

#[wasm_bindgen]
pub struct PublishParams(RPublishParams);

impl From<RPublishParams> for PublishParams {
    fn from(v: RPublishParams) -> PublishParams {
        PublishParams(v)
    }
}

impl From<PublishParams> for RPublishParams {
    fn from(v: PublishParams) -> RPublishParams {
        v.0
    }
}

#[wasm_bindgen]
impl PublishParams {
    #[wasm_bindgen(getter)]
    pub fn token(&self) -> Option<String> {
        self.0.token.clone()
    }
}

#[wasm_bindgen]
impl TwetchPay {
    pub async fn run(value: JsValue, wallet: Wallet) -> Result<TwetchPayAction, JsValue> {
        let mut call = match value.into_serde::<twetch_pay::TwetchPayCall>() {
            Ok(v) => v,
            Err(e) => {
                let payload = json!({
                    "status": 0,
                    "message": format!("{}", e),
                });
                return Err(JsValue::from_serde(&payload).unwrap());
            }
        };

        let pay = twetch_pay::TwetchPay {
            wallet: wallet.into(),
        };

        let action = match pay.run(&call).await {
            Ok(v) => v,
            Err(e) => {
                let payload = json!({
                    "status": 1,
                    "message": format!("{}", e),
                });
                return Err(JsValue::from_serde(&payload).unwrap());
            }
        };

        Ok(action.into())
    }

    pub async fn submit(action: TwetchPayAction, wallet: Wallet) -> Result<PublishParams, JsValue> {
        let pay = twetch_pay::TwetchPay {
            wallet: wallet.into(),
        };

        let params = match pay.submit(&action.into()).await {
            Ok(v) => v,
            Err(e) => {
                let payload = json!({
                    "message": format!("{:?}", e),
                });
                return Err(JsValue::from_serde(&payload).unwrap());
            }
        };

        Ok(params.into())
    }
}
