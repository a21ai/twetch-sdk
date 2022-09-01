use crate::{
    BuiltTx, Networks, PublishParams, SigilAction, TxBuilderOutput, V1TwetchAction, Wallet,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub struct TwetchPay {
    pub wallet: Wallet,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TwetchPayActionType {
    Sigil,
    Twetch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayParams {
    #[serde(rename(deserialize = "tweetFromTwetch", serialize = "tweetFromTwetch"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_from_twetch: Option<bool>,
    #[serde(rename(deserialize = "encryptedHash", serialize = "encryptedHash"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_hash: Option<String>,
    #[serde(rename(
        deserialize = "filesEncryptedHashes",
        serialize = "filesEncryptedHashes"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_encrypted_hashes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwetchPayCall {
    pub action_type: TwetchPayActionType,
    pub network: Networks,
    pub action: Option<String>,
    pub args: Option<Map<String, Value>>,
    pub outputs: Option<Vec<TxBuilderOutput>>,
    #[serde(rename(deserialize = "payParams", serialize = "payParams"))]
    pub pay_params: Option<PayParams>,
    pub encrypt_args: Option<bool>,
}

pub struct TwetchPayAction {
    pub built_tx: BuiltTx,
    pub call: TwetchPayCall,
    pub is_troll_toll: Option<bool>,
}

impl TwetchPayAction {}

impl TwetchPay {
    pub async fn run(&self, call: &TwetchPayCall) -> Result<TwetchPayAction> {
        match call.action_type {
            TwetchPayActionType::Twetch => Ok(V1TwetchAction::run(&self.wallet, call).await?),
            TwetchPayActionType::Sigil => Ok(SigilAction::run(&self.wallet, call).await?),
        }
    }

    pub async fn submit(&self, action: &TwetchPayAction) -> Result<PublishParams> {
        Ok(V1TwetchAction::submit(&self.wallet, action).await?)
    }
}
