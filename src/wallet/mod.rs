pub mod networks;
pub mod tx_builder;
pub mod utxo;

pub use networks::*;
pub use tx_builder::*;
pub use utxo::*;

use crate::{constants, AuthToken, MetasyncApi};
use anyhow::Result;
use bsv::{
    ChainParams, ExtendedPrivateKey, ExtendedPublicKey, P2PKHAddress, PrivateKey, PublicKey,
    Script, SigHash, Transaction, BSM, ECIES,
};
use sigil_types::TypedSigning;

#[derive(Clone)]
pub struct Wallet {
    seed: String,
    pub user_id: Option<String>,
    pub token: Option<String>,
}

pub struct EphemeralCipher {
    pub cipher_text: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Wallet {
    pub fn new(seed: String) -> Wallet {
        Wallet {
            seed,
            user_id: None,
            token: None,
        }
    }

    pub fn from_seed_and_token(seed: String, token: String) -> Result<Wallet> {
        let auth_token = AuthToken::new(token.clone())?;

        Ok(Wallet {
            seed,
            user_id: Some(auth_token.user_id),
            token: Some(token),
        })
    }

    fn xpriv(&self) -> Result<ExtendedPrivateKey> {
        Ok(ExtendedPrivateKey::from_mnemonic(
            self.seed.as_bytes(),
            None,
        )?)
    }

    pub fn xpub(&self) -> Result<ExtendedPublicKey> {
        Ok(ExtendedPublicKey::from_xpriv(&self.xpriv()?))
    }

    fn account_xpriv(&self) -> Result<ExtendedPrivateKey> {
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

    pub async fn account_balance(&self, network: &Networks) -> Result<u64> {
        Ok(self
            .account_utxos(network)
            .await?
            .iter()
            .map(|e| e.satoshis)
            .sum())
    }

    fn wallet_xpriv(&self) -> Result<ExtendedPrivateKey> {
        Ok(self.xpriv()?.derive_from_path("m/44'/0'/0'/0")?)
    }

    pub fn wallet_xpub(&self) -> Result<ExtendedPublicKey> {
        Ok(ExtendedPublicKey::from_xpriv(&self.wallet_xpriv()?))
    }

    pub fn display_address(&self, network: &Networks) -> Result<String> {
        let address = self.account_address()?;
        let params = match network {
            Networks::BSV => ChainParams::mainnet(),
            Networks::TBSV => ChainParams::testnet(),
        };
        Ok(address.set_chain_params(&params)?.to_string()?)
    }

    pub async fn wallet_utxos(&self, network: &Networks) -> Result<Vec<UTXO>> {
        Ok(UTXO::from_metasync(&self.account_public_key()?, network).await?)
    }

    pub fn ephemeral_encrypt(&self, plain_text: Vec<u8>) -> Result<EphemeralCipher> {
        let account_public_key = self.account_public_key()?;
        let random_private_key = PrivateKey::from_random();

        let cipher_text =
            ECIES::encrypt(&plain_text, &random_private_key, &account_public_key, false)?
                .to_bytes();
        let cipher_keys = ECIES::derive_cipher_keys(&random_private_key, &account_public_key)?;

        let mut hash = Vec::new();
        hash.append(&mut cipher_keys.get_iv());
        hash.append(&mut cipher_keys.get_ke());
        hash.append(&mut cipher_keys.get_km());

        Ok(EphemeralCipher { cipher_text, hash })
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

    pub async fn resolve_change_address(&self) -> Result<Script> {
        if let Some(user_id) = &self.user_id {
            let metasync = MetasyncApi::new(constants::METASYNC_URL.to_string());
            let payment_destination = metasync
                .payment_destination(&format!("{}@twetch.me", user_id))
                .await?;
            return Ok(Script::from_hex(&payment_destination.outputs[0].script)?);
        }
        return Ok(self.account_address()?.get_locking_script()?);
    }

    pub fn sign_message(&self, message: String) -> Result<String> {
        let private_key = self.account_xpriv()?.get_private_key();
        let sig = BSM::sign_message(&private_key, message.as_bytes())?.to_compact_bytes(None);
        Ok(base64::encode_config(sig, base64::STANDARD))
    }

    pub fn sign_typed(&self, typed_signing: &mut TypedSigning) -> Result<TypedSigning> {
        let private_key_account = self.account_private_key()?;
        let mut private_keys = Vec::new();

        for _ in &typed_signing.signatures {
            private_keys.push(private_key_account.clone());
        }

        Ok(typed_signing.sign_all(private_keys, Vec::new())?)
    }

    pub fn sign_transaction(&self, tx: &mut Transaction, utxos: &Vec<Option<UTXO>>) -> Result<()> {
        let xpriv_wallet = self.wallet_xpriv()?;
        let private_key_account = self.account_private_key()?;

        for i in 0..tx.get_ninputs() {
            if let Some(utxo) = &utxos[i] {
                let mut input = match tx.get_input(i) {
                    Some(v) => v,
                    None => continue,
                };

                let private_key = if utxo.path == -1 {
                    private_key_account.clone()
                } else {
                    xpriv_wallet.derive(utxo.path as u32)?.get_private_key()
                };

                //if utxo.script.is_none() {
                //anyhow::bail!(format!("Missing Locking Script {:?}", utxo));
                //}

                //// Sigil V2
                //let signature = tx.sign(
                //&private_key,
                //SigHash::InputOutput,
                //i,
                //&utxo.script.clone().unwrap(),
                //utxo.satoshis,
                //)?;
                //let asm = format!(
                //"{} {} {}",
                //signature.to_hex()?,
                //private_key.to_public_key()?.to_hex()?,
                //contract.clone()
                //);
                //input.set_locking_script(&utxo.script.clone().unwrap());
                //input.set_unlocking_script(&Script::from_asm_string(&asm)?);
                //tx.set_input(i, &input)

                // P2PKH
                let signature = tx.sign(
                    &private_key,
                    SigHash::InputsOutputs,
                    i,
                    &private_key
                        .to_public_key()?
                        .to_p2pkh_address()?
                        .get_locking_script()?,
                    utxo.satoshis,
                )?;

                let asm = format!(
                    "{} {}",
                    signature.to_hex()?,
                    private_key.to_public_key()?.to_hex()?
                );
                input.set_locking_script(
                    &private_key
                        .to_public_key()?
                        .to_p2pkh_address()?
                        .get_locking_script()?,
                );
                input.set_unlocking_script(&Script::from_asm_string(&asm)?);
                tx.set_input(i, &input)
            }
        }

        Ok(())
    }

    pub async fn resolve_output(
        &self,
        output: &TxBuilderOutput,
    ) -> Result<TxBuilderResolvedOutput> {
        TxBuilder::resolve_output(output, self).await
    }

    pub async fn build_tx(&self, builder: &TxBuilder) -> Result<BuiltTx> {
        TxBuilder::build(builder, self).await
    }
}
