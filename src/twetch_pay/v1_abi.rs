use crate::ABIv1Schema;
use bsv::Hash;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ABIv1ArgType {
    Address,
    String,
    Signature,
}

fn default_abi_v2_arg_type() -> ABIv1ArgType {
    ABIv1ArgType::String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ABIv1Arg {
    pub name: String,
    #[serde(rename(deserialize = "type",))]
    #[serde(default = "default_abi_v2_arg_type")]
    pub arg_type: ABIv1ArgType,
    pub value: Option<String>,
    #[serde(rename(deserialize = "replaceValue",))]
    pub replace_value: Option<String>,
    #[serde(rename(deserialize = "defaultValue",))]
    pub default_value: Option<String>,
    #[serde(rename(deserialize = "messageStartIndex",))]
    pub message_start_index: Option<usize>,
    #[serde(rename(deserialize = "messageEndIndex",))]
    pub message_end_index: Option<usize>,
    #[serde(rename(deserialize = "addressIndex",))]
    pub address_index: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ABIv1Action {
    pub args: Vec<ABIv1Arg>,
}

pub struct ABIv1 {
    pub args: Vec<String>,
    pub action: ABIv1Action,
}

impl ABIv1 {
    pub fn to_array(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn to_chunks(&self) -> Vec<Vec<u8>> {
        self.args.iter().map(|e| e.as_bytes().to_vec()).collect()
    }

    pub fn replace(&mut self, replace: String, value: String) -> Result<()> {
        let index: usize = match self.action.args.iter().position(|e| {
            if let Some(v) = &e.replace_value {
                return v.to_string() == replace;
            }
            return false;
        }) {
            Some(v) => v,
            None => return Ok(()),
        };

        self.args[index] = value;

        Ok(())
    }

    pub fn from_action(action: &String) -> Result<ABIv1> {
        let schema = ABIv1Schema::new()?;
        let action = match schema.actions.get(action) {
            Some(v) => v.clone(),
            None => {
                anyhow::bail!("ABI Error: action not found in abi shcema")
            }
        };

        Ok(ABIv1 {
            action,
            args: Vec::new(),
        })
    }

    pub fn from_object(action: &String, object: &Map<String, Value>) -> Result<ABIv1> {
        let mut abi = ABIv1::from_action(action)?;

        abi.args = abi
            .action
            .args
            .iter()
            .map(|e| {
                if let Some(v) = object.get(&e.name) {
                    return v.as_str().unwrap().to_string();
                }

                if let Some(v) = &e.value {
                    return v.to_string();
                }

                if let Some(v) = &e.replace_value {
                    return v.to_string();
                }

                if let Some(v) = &e.default_value {
                    return v.to_string();
                }

                "".to_string()
            })
            .collect();

        Ok(abi)
    }

    pub fn content_hash(&self) -> Result<String> {
        let arg = match self
            .action
            .args
            .iter()
            .find(|e| ABIv1ArgType::Signature == e.arg_type)
        {
            Some(v) => v,
            None => anyhow::bail!("ABI Error: signature not found"),
        };

        let start_index = match arg.message_start_index {
            Some(v) => v,
            None => 0,
        };

        let end_index = match arg.message_end_index {
            Some(v) => v + 1,
            None => 0,
        };

        let value = &self.to_chunks()[start_index..end_index];

        let mut data = Vec::new();

        for v in value {
            data.append(&mut v.clone());
        }

        Ok(Hash::sha_256(&data).to_hex())
    }
}
