use bsv_wasm::{ExtendedPrivateKey, P2PKHAddress, PublicKey, Transaction};
use twetch_sdk::{networks, wallet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wallet(wallet::Wallet);

#[wasm_bindgen]
pub enum Networks {
    BSV,
    TBSV,
}

fn get_network(n: Networks) -> networks::Networks {
    match n {
        Networks::BSV => networks::Networks::BSV,
        Networks::TBSV => networks::Networks::TBSV,
    }
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: String) -> Wallet {
        Wallet(wallet::Wallet::new(seed))
    }

    pub fn xpriv_account(&self) -> Option<ExtendedPrivateKey> {
        match wallet::Wallet::xpriv_account(&self.0) {
            Ok(v) => Some(ExtendedPrivateKey::from(v)),
            Err(_) => None,
        }
    }

    pub fn xpriv_wallet(&self) -> Option<ExtendedPrivateKey> {
        match wallet::Wallet::xpriv_wallet(&self.0) {
            Ok(v) => Some(ExtendedPrivateKey::from(v)),
            Err(_) => None,
        }
    }

    pub fn public_key_account(&self) -> Option<PublicKey> {
        match wallet::Wallet::public_key_account(&self.0) {
            Ok(v) => Some(PublicKey::from(v)),
            Err(_) => None,
        }
    }

    pub fn address_account(&self) -> Option<P2PKHAddress> {
        match wallet::Wallet::address_account(&self.0) {
            Ok(v) => Some(P2PKHAddress::from(v)),
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
        match wallet::Wallet::sign_transaction(&self.0, &transaction.into(), &Vec::new()) {
            Ok(v) => Some(Transaction::from(v)),
            Err(_) => None,
        }
    }

    pub async fn balance(&self, network: Networks) -> Option<u64> {
        match wallet::Wallet::balance(&self.0, &get_network(network)).await {
            Ok(v) => Some(v),
            Err(_) => return None,
        }
    }
}
