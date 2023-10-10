use crate::{constants, Networks, PolynymApi, TxlogApi, Wallet, UTXO};
use anyhow::Result;
use bsv::{P2PKHAddress, Script, Transaction, TxIn, TxOut, VarInt};
use serde::{Deserialize, Serialize};

use sigil_types::TypedSigning;

#[derive(Debug, Serialize, Deserialize)]
pub struct NFT {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxBuilderOutput {
    pub sats: u64,
    pub address: Option<String>,
    pub to: Option<String>,
    pub script: Option<String>,
    pub args: Option<Vec<String>>,
    pub encrypt_args: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxBuilderResolvedOutput {
    pub tx_outs: Vec<TxOut>,
    pub payment_destination: Option<PaymentDestination>,
    pub encrypted_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentDestination {
    pub paymail: String,
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxBuilder {
    pub network: Networks,
    pub contract: Option<String>,
    pub extended_tx: Option<String>,
    pub typed_signing: Option<TypedSigning>,
    pub outputs: Vec<TxBuilderOutput>,
    pub change_address: Option<P2PKHAddress>,
    pub auto_fund: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuiltTx {
    pub tx: Transaction,
    pub txid: String,
    pub total_cost_sats: u64,
    pub fee_sats: u64,
    pub payment_destinations: Vec<PaymentDestination>,
    pub encrypted_hash: Option<String>,
    pub nfts: Vec<String>,
    pub typed_signing: Option<TypedSigning>,
}

impl TxBuilder {
    pub fn estimate_cost(tx: &Transaction, wallet: &Wallet) -> Result<u64> {
        let mut cost = 0;

        for i in 0..tx.get_noutputs() {
            let tx_out = tx.get_output(i).unwrap();

            if !tx_out
                .get_script_pub_key_hex()
                .contains(&wallet.account_locking_script()?.to_hex())
            {
                cost = cost + tx_out.get_satoshis();
            }
        }

        Ok(cost)
    }

    pub fn estimate_size(tx: &Transaction, utxos: &Vec<Option<UTXO>>) -> Result<usize> {
        let mut size = constants::TX_VERSION_SIZE;
        size = size + VarInt::get_varint_size((tx.get_ninputs()) as u64);

        size = size
            + utxos
                .iter()
                .map(|e| {
                    let script_size = match e {
                        Some(_) => constants::SIGIL_V2_UNLOCKING_SCRIPT_SIZE,
                        None => constants::P2PKH_UNLOCKING_SCRIPT_SIZE,
                    };

                    constants::TXIN_TXID_SIZE
                        + constants::TXIN_VOUT_SIZE
                        + VarInt::get_varint_size(script_size as u64)
                        + script_size
                        + constants::TXIN_SEQUENCE_SIZE
                })
                .sum::<usize>();

        size = size + VarInt::get_varint_size((tx.get_noutputs() + 1) as u64); // +1 for change address

        for i in 0..tx.get_noutputs() {
            let tx_out = tx.get_output(i).unwrap();
            size = size + tx_out.to_bytes()?.len();
        }

        // change addres
        size = size + constants::P2PKH_OUTPUT_SIZE;

        size = size + constants::TX_LOCKTIME_SIZE;

        Ok(size)
    }

    pub async fn resolve_output(
        output: &TxBuilderOutput,
        wallet: &Wallet,
    ) -> Result<TxBuilderResolvedOutput> {
        let mut tx_outs = Vec::new();
        let mut encrypted_hash = None;
        let mut payment_destination = None;

        if let Some(address) = &output.address {
            tx_outs.push(TxOut::new(
                output.sats,
                &P2PKHAddress::from_string(address)?.get_locking_script()?,
            ));
        } else if let Some(to) = &output.to {
            let polynym = PolynymApi::new(constants::POLYNYM_URL.to_string());

            // Addresses
            if let Ok(address) = P2PKHAddress::from_string(to) {
                tx_outs.push(TxOut::new(output.sats, &address.get_locking_script()?));
            } else if to.contains("@") && !to.starts_with("@") {
                // User Paymails
                if let Ok(p2p_payment_destination) =
                    polynym.p2p_payment_destination(to, output.sats).await
                {
                    payment_destination = Some(PaymentDestination {
                        paymail: to.clone(),
                        reference: p2p_payment_destination.reference,
                    });

                    for o in p2p_payment_destination.outputs.iter() {
                        tx_outs.push(TxOut::new(o.satoshis, &Script::from_hex(&o.script)?));
                    }
                } else {
                    if let Ok(address) = polynym.get_address(to).await {
                        tx_outs.push(TxOut::new(output.sats, &address.get_locking_script()?));
                    }
                }
            } else {
                // User Number
                let search = match to.parse::<u64>() {
                    Ok(v) => format!("@{}", v),
                    Err(_) => to.clone(),
                };

                if let Ok(address) = polynym.get_address(&search).await {
                    tx_outs.push(TxOut::new(output.sats, &address.get_locking_script()?));
                }
            }
        } else if let Some(script_string) = &output.script {
            let script = match Script::from_asm_string(&script_string) {
                Ok(v) => v,
                Err(_) => anyhow::bail!("failed to resolve 'script'"),
            };
            tx_outs.push(TxOut::new(output.sats, &script));
        } else if let Some(args) = &output.args {
            let asm = args
                .iter()
                .map(|a| hex::encode(a.as_bytes()))
                .collect::<Vec<String>>()
                .join(" ");

            if let Some(encrypt) = output.encrypt_args {
                if encrypt {
                    let script = Script::from_asm_string(&asm)?.to_bytes();
                    let cipher = wallet.ephemeral_encrypt(script)?;
                    encrypted_hash = Some(hex::encode(cipher.hash));
                    tx_outs.push(TxOut::new(
                        0,
                        &Script::from_asm_string(&format!(
                            "0 OP_RETURN 747765746368 {}",
                            hex::encode(cipher.cipher_text)
                        ))?,
                    ))
                } else {
                    tx_outs.push(TxOut::new(
                        0,
                        &Script::from_asm_string(&format!("0 OP_RETURN {}", asm))?,
                    ))
                }
            } else {
                tx_outs.push(TxOut::new(
                    0,
                    &Script::from_asm_string(&format!("0 OP_RETURN {}", asm))?,
                ))
            }
        }

        Ok(TxBuilderResolvedOutput {
            tx_outs,
            payment_destination,
            encrypted_hash,
        })
    }

    pub fn find_nft(tx_out: TxOut) -> Option<NFT> {
        let data = match tx_out
            .get_script_pub_key()
            .to_asm_string()
            .split(" ")
            .last()
        {
            Some(v) => v.to_string(),
            None => return None,
        };

        let bytes = match hex::decode(&data) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let utf8 = match std::str::from_utf8(&bytes) {
            Ok(v) => v.to_string(),
            Err(_) => return None,
        };

        let nft: NFT = match serde_json::from_str(&utf8) {
            Ok(v) => v,
            Err(_) => return None,
        };

        Some(nft)
    }

    pub async fn build(builder: &TxBuilder, wallet: &Wallet) -> Result<BuiltTx> {
        let mut output_sats = 0_u64;
        let mut input_sats = 0_u64;
        let mut utxos: Vec<Option<UTXO>> = Vec::new();
        let mut payment_destinations: Vec<PaymentDestination> = Vec::new();
        let mut encrypted_hash = None;
        let mut nfts: Vec<String> = Vec::new();
        let mut typed_signing: Option<TypedSigning> = None;

        let change_script = match &builder.change_address {
            Some(v) => v.get_locking_script()?,
            None => wallet.account_locking_script()?,
        };

        let mut tx = match &builder.extended_tx {
            Some(v) => Transaction::from_compact_hex(v)?,
            None => Transaction::default(),
        };

        if let Some(typed_signing) = &builder.typed_signing {
            tx = Transaction::from_compact_bytes(&typed_signing.data)?;
        };

        // Add existing satoshis to transaction
        for i in 0..tx.get_noutputs() {
            let tx_out = tx.get_output(i).unwrap();

            match TxBuilder::find_nft(tx_out.clone()) {
                Some(v) => {
                    output_sats += tx_out.get_satoshis();
                    nfts.push(v.title)
                }
                None => {
                    output_sats += tx_out.get_satoshis();
                }
            };
        }

        // TODO : make this work for many privs
        for i in 0..tx.get_ninputs() {
            let tx_in = tx.get_input(i).unwrap();

            let txlog_api = TxlogApi::new(constants::urls::TXLOG_URL.to_string());
            let satoshis_in = match txlog_api
                .satoshis(&tx_in.get_prev_tx_id_hex(None), tx_in.get_vout())
                .await
            {
                Ok(v) => v,
                Err(_) => anyhow::bail!("failed to fetch prev tx"),
            };

            input_sats += satoshis_in;

            utxos.push(None);
        }

        for output in &builder.outputs {
            let res = match TxBuilder::resolve_output(output, wallet).await {
                Ok(v) => v,
                Err(_) => anyhow::bail!("failed to resolve output"),
            };

            if let Some(p) = res.payment_destination {
                payment_destinations.push(p);
            }

            if let Some(h) = res.encrypted_hash {
                encrypted_hash = Some(h);
            }

            output_sats += output.sats;
            for tx_out in &res.tx_outs {
                tx.add_output(tx_out);
            }
        }

        if builder.auto_fund == true {
            let wallet_utxos = match wallet.utxos(&builder.network, output_sats + 100000).await {
                Ok(v) => v,
                Err(_) => anyhow::bail!("failed to fetch utxos"),
            };

            let balance: u64 = wallet_utxos.iter().map(|e| e.satoshis).sum();
            anyhow::ensure!(
                balance >= output_sats,
                format!(
                    "Insufficient wallet balance : {:.8} BSV - {:.8} BSV",
                    balance as f64 / 1e8,
                    output_sats as f64 / 1e8
                )
            );

            for utxo in &wallet_utxos {
                if input_sats < output_sats + 100000 {
                    input_sats += utxo.satoshis;
                    utxos.push(Some(utxo.clone()));
                    tx.add_input(&TxIn::new(
                        &hex::decode(utxo.txid.clone()).unwrap(),
                        utxo.vout,
                        &Script::default(),
                        None,
                    ))
                } else {
                    break;
                }
            }
        }

        let size = TxBuilder::estimate_size(&tx, &utxos)?;
        let fee_sats: u64 = ((size as f64) * constants::TX_FEE_RATE).ceil() as u64;
        let change_sats: i64 = input_sats as i64 - output_sats as i64 - fee_sats as i64;

        let total_cost_sats = fee_sats + TxBuilder::estimate_cost(&tx, &wallet)?;

        if builder.auto_fund == true {
            if change_sats > 0 {
                tx.add_output(&TxOut::new(change_sats as u64, &change_script));
            } else {
                anyhow::bail!("Insufficient transaction fees");
            }
        }

        wallet.sign_transaction(&mut tx, &utxos)?;

        if let Some(ts) = &builder.typed_signing {
            let ts = wallet.sign_typed(&mut ts.clone())?;

            for signature in &ts.signatures {
                if let Some(sig) = &signature.signature {
                    let asm = format!(
                        "{} {} {}",
                        hex::encode(sig.clone()),
                        wallet.account_public_key()?.to_hex()?,
                        builder.contract.clone().unwrap()
                    );

                    let vin = signature.vin.unwrap();
                    let mut input = tx.get_input(vin).unwrap();
                    input.set_unlocking_script(&Script::from_asm_string(&asm)?);
                    tx.set_input(vin, &input);
                }
            }

            typed_signing = Some(TypedSigning {
                data: tx.to_compact_bytes()?,
                signatures: ts.signatures.clone(),
            });
        }

        let fee_rate = fee_sats as f64 / tx.get_size().unwrap() as f64;
        if fee_rate < constants::MIN_TX_FEE_RATE {
            anyhow::bail!("Fee rate too low ({:.3} sats/byte)", fee_rate);
        }

        Ok(BuiltTx {
            tx: tx.clone(),
            txid: tx.get_id_hex()?.clone(),
            encrypted_hash,
            total_cost_sats,
            fee_sats,
            payment_destinations,
            nfts,
            typed_signing,
        })
    }
}
