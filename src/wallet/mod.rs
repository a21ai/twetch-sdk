pub mod networks;
pub mod utxo;

pub use networks::*;
pub use utxo::*;

use crate::{constants, MetasyncApi};
use anyhow::Result;
use bsv::{
    ExtendedPrivateKey, ExtendedPublicKey, P2PKHAddress, PrivateKey, PublicKey, Script, SigHash,
    Transaction, BSM,
};

pub struct Wallet {
    seed: String,
}

impl Wallet {
    pub fn new(seed: String) -> Wallet {
        Wallet { seed }
    }

    pub fn xpriv(&self) -> Result<ExtendedPrivateKey> {
        Ok(ExtendedPrivateKey::from_mnemonic(
            self.seed.as_bytes(),
            None,
        )?)
    }

    pub fn xpub(&self) -> Result<ExtendedPublicKey> {
        Ok(ExtendedPublicKey::from_xpriv(&self.xpriv()?))
    }

    pub fn account_xpriv(&self) -> Result<ExtendedPrivateKey> {
        Ok(self.xpriv()?.derive_from_path("m/0/0")?)
    }

    pub fn account_private_key(&self) -> Result<PrivateKey> {
        Ok(self.account_xpriv()?.get_private_key())
    }

    pub fn account_public_key(&self) -> Result<PublicKey> {
        Ok(self.account_private_key()?.to_public_key()?)
    }

    pub fn account_address(&self) -> Result<P2PKHAddress> {
        Ok(self.account_public_key()?.to_p2pkh_address()?)
    }

    pub async fn account_utxos(&self, network: &Networks) -> Result<Vec<UTXO>> {
        Ok(UTXO::from_woc(&self.account_address()?, network).await?)
    }

    pub fn wallet_xpriv(&self) -> Result<ExtendedPrivateKey> {
        Ok(self.xpriv()?.derive_from_path("m/44'/0'/0'/0")?)
    }

    pub fn wallet_xpub(&self) -> Result<ExtendedPublicKey> {
        Ok(ExtendedPublicKey::from_xpriv(&self.wallet_xpriv()?))
    }

    pub async fn wallet_utxos(&self, network: &Networks) -> Result<Vec<UTXO>> {
        Ok(UTXO::from_metasync(&self.account_public_key()?, network).await?)
    }

    pub fn sign_message(&self, message: String) -> Result<String> {
        let private_key = self.account_xpriv()?.get_private_key();
        let sig = BSM::sign_message(&private_key, message.as_bytes())?.to_compact_bytes(None);
        Ok(base64::encode_config(sig, base64::STANDARD))
    }

    pub async fn balance(&self, network: &Networks) -> Result<u64> {
        Ok(self.utxos(network).await?.iter().map(|e| e.satoshis).sum())
    }

    pub async fn utxos(&self, network: &Networks) -> Result<Vec<UTXO>> {
        let wallet_utxos = self.wallet_utxos(network);
        let account_utxos = self.account_utxos(network);

        let mut wallet_utxos = wallet_utxos.await?;
        let mut account_utxos = account_utxos.await?;

        wallet_utxos.append(&mut account_utxos);

        Ok(wallet_utxos)
    }

    pub async fn resolve_change_address(&self, user_id: String) -> Result<Script> {
        let metasync = MetasyncApi::new(constants::METASYNC_URL.to_string());
        let payment_destination = metasync
            .payment_destination(&format!("{}@twetch.me", user_id))
            .await?;
        Ok(Script::from_hex(&payment_destination.outputs[0].script)?)
    }

    pub fn sign_transaction(&self, tx: &Transaction, utxos: &Vec<UTXO>) -> Result<Transaction> {
        let mut tx = tx.clone();

        let xpriv_wallet = self.wallet_xpriv()?;
        let private_key_account = self.account_private_key()?;

        for i in 0..tx.get_ninputs() {
            let utxo = &utxos[i];

            let mut input = match tx.get_input(i) {
                Some(v) => v,
                None => continue,
            };

            let locking_script = match input.get_locking_script() {
                Some(v) => v,
                None => continue,
            };

            let private_key = if utxo.path == -1 {
                private_key_account.clone()
            } else {
                xpriv_wallet.derive(utxo.path as u32)?.get_private_key()
            };

            let private_key_locking_script = private_key
                .to_public_key()?
                .to_p2pkh_address()?
                .get_locking_script()?
                .to_hex();

            if private_key_locking_script == locking_script.to_hex() {
                let signature = tx.sign(
                    &private_key,
                    SigHash::InputsOutputs,
                    i,
                    &locking_script,
                    utxo.satoshis,
                )?;

                let asm = format!(
                    "{} {}",
                    signature.to_hex()?,
                    private_key.to_public_key()?.to_hex()?
                );
                input.set_unlocking_script(&Script::from_asm_string(&asm)?);
                tx.set_input(i, &input)
            } else {
                let signature = tx.sign(
                    &private_key,
                    SigHash::InputOutput,
                    i,
                    &utxo.script.clone().unwrap(),
                    utxo.satoshis,
                )?;
                let asm = format!(
                    "{} {} {}",
                    signature.to_hex()?,
                    private_key.to_public_key()?.to_hex()?,
                    utxo.contract.clone().unwrap()
                );
                input.set_unlocking_script(&Script::from_asm_string(&asm)?);
                tx.set_input(i, &input)
            }
        }

        Ok(tx.clone())
    }
}
