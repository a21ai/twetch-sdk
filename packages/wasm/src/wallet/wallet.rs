use crate::{EphemeralCipher, TypedSigning};
use bsv_wasm::{ExtendedPrivateKey, ExtendedPublicKey, P2PKHAddress, PublicKey, Transaction};
use twetch_sdk::{wallet, UTXO};
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
pub struct Wallet(wallet::Wallet);

impl From<wallet::Wallet> for Wallet {
    fn from(v: wallet::Wallet) -> Wallet {
        Wallet(v)
    }
}

impl From<Wallet> for wallet::Wallet {
    fn from(v: Wallet) -> wallet::Wallet {
        v.0
    }
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: String) -> Wallet {
        Wallet(wallet::Wallet::new(seed))
    }

    pub fn from_seed_and_token(seed: String, token: String) -> Option<Wallet> {
        match wallet::Wallet::from_seed_and_token(seed, token) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn xpub(&self) -> Option<ExtendedPublicKey> {
        match wallet::Wallet::xpub(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn account_address(&self) -> Option<P2PKHAddress> {
        match wallet::Wallet::account_address(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn account_public_key(&self) -> Option<PublicKey> {
        match wallet::Wallet::account_public_key(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn wallet_xpriv(&self) -> Option<ExtendedPrivateKey> {
        match wallet::Wallet::wallet_xpriv(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn wallet_xpub(&self) -> Option<ExtendedPublicKey> {
        match wallet::Wallet::wallet_xpub(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn taproot_xpriv(&self) -> Option<ExtendedPrivateKey> {
        match wallet::Wallet::taproot_xpriv(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn taproot_xpub(&self) -> Option<ExtendedPublicKey> {
        match wallet::Wallet::taproot_xpub(&self.0) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn display_address(&self, network: String) -> Option<String> {
        match wallet::Wallet::display_address(&self.0, &network.into()) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    //pub fn display_address_segwit(&self) -> Option<String> {
    //match wallet::Wallet::display_address_segwit(&self.0) {
    //Ok(v) => Some(v.into()),
    //Err(_) => None,
    //}
    //}

    pub fn ephemeral_encrypt(&self, plain_text: Vec<u8>) -> Option<EphemeralCipher> {
        match wallet::Wallet::ephemeral_encrypt(&self.0, plain_text) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn sign_typed(&self, typed_signing: TypedSigning) -> Option<TypedSigning> {
        match wallet::Wallet::sign_typed(&self.0, &mut typed_signing.into()) {
            Ok(v) => Some(v.into()),
            Err(_) => None,
        }
    }

    pub fn sign_message(&self, message: String) -> Option<String> {
        match wallet::Wallet::sign_message(&self.0, message) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn sign_transaction(&self, transaction: Transaction) -> Option<Transaction> {
        let mut tx = transaction.into();
        match wallet::Wallet::sign_transaction(&self.0, &mut tx, &Vec::new()) {
            Ok(_) => Some(tx.into()),
            Err(_) => None,
        }
    }

    pub async fn utxos(account_public_key: PublicKey, network: String) -> Result<JsValue, JsValue> {
        let mut utxos = Vec::new();

        let address = match account_public_key.to_p2pkh_address() {
            Ok(v) => v,
            Err(_v) => return Err(JsValue::null()),
        };

        match UTXO::from_woc(&address.into(), &network.clone().into()).await {
            Ok(mut v) => utxos.append(&mut v),
            Err(_) => {}
        };

        match UTXO::from_metasync(&account_public_key.into(), &network.into()).await {
            Ok(mut v) => utxos.append(&mut v),
            Err(_) => {}
        };

        Ok(JsValue::from_serde(&serde_json::to_value(utxos).unwrap()).unwrap())
    }

    pub async fn account_utxos(
        account_address: P2PKHAddress,
        network: String,
    ) -> Result<JsValue, JsValue> {
        match UTXO::from_woc(&account_address.into(), &network.into()).await {
            Ok(v) => Ok(JsValue::from_serde(&serde_json::to_value(v).unwrap()).unwrap()),
            Err(_) => Ok(JsValue::null()),
        }
    }

    pub async fn wallet_utxos(
        account_public_key: PublicKey,
        network: String,
    ) -> Result<JsValue, JsValue> {
        match UTXO::from_metasync(&account_public_key.into(), &network.into()).await {
            Ok(v) => Ok(JsValue::from_serde(&serde_json::to_value(v).unwrap()).unwrap()),
            Err(_) => Ok(JsValue::null()),
        }
    }

    pub async fn account_balance(
        account_address: P2PKHAddress,
        network: String,
    ) -> Result<JsValue, JsValue> {
        match UTXO::from_woc(&account_address.into(), &network.into()).await {
            Ok(v) => {
                let sum: u64 = v.iter().map(|e| e.satoshis).sum();
                Ok(JsValue::from_serde(&serde_json::to_value(sum).unwrap()).unwrap())
            }
            Err(_) => Ok(JsValue::null()),
        }
    }
}
