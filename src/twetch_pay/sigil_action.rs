use crate::{
    constants, get_mint_utos, get_uto, BitcoinFilesApi, Networks, SigilABI, SigilABIMethods,
    TwetchPayAction, TwetchPayCall, TxBuilder, TxBuilderOutput, Wallet,
};
use anyhow::Result;

use async_trait::async_trait;
use sigil_sdk::contracts::{brc721, lizervaxx, slurp_juice};
use sigil_types::{Outpoint, SigilError, TokenStore, UTO, UTXO};

pub struct SigilAction {}

pub struct TestStore {}

#[async_trait]
impl TokenStore for TestStore {
    async fn get_uto(&self, _outpoint: &Outpoint) -> Result<UTO, SigilError> {
        Err(SigilError::UTONotFound(
            "get_uto not implemented".to_string(),
        ))
    }

    async fn get_utxo(&self, _outpoint: &Outpoint) -> Result<UTXO, SigilError> {
        Err(SigilError::UTONotFound(
            "get_utxo not implemented".to_string(),
        ))
    }
}

impl SigilAction {
    pub async fn run(wallet: &Wallet, call: &TwetchPayCall) -> Result<TwetchPayAction> {
        let mut outputs: Vec<TxBuilderOutput> = Vec::new();
        let store = TestStore {};

        if let Some(args) = &call.args {
            let abi: SigilABI = serde_json::from_value(serde_json::to_value(args)?)?;

            let mut auto_fund = true;

            let params = &abi.params;

            let typed_signing = match abi.method {
                SigilABIMethods::Transfer => {
                    let contract = brc721::BRC721Basic {
                        store: Box::new(store),
                    };
                    let uto = get_uto(params[0].clone().into()).await?;
                    contract.abi(brc721::basic::ABI::Transfer(uto, params[1].clone().into()))?
                }
                SigilABIMethods::Mint => {
                    let contract = brc721::BRC721Basic {
                        store: Box::new(store),
                    };

                    let utos = get_mint_utos(params[0].clone(), abi.contract.clone());

                    let utxo = UTXO {
                        outpoint: params[1].clone().into(),
                        satoshis: params[2].clone().into(),
                        script: params[3].clone().into(),
                    };

                    contract.abi(brc721::basic::ABI::Mint(utxo, utos))?
                }
                SigilABIMethods::Purchase => {
                    let contract = brc721::BRC721Basic {
                        store: Box::new(store),
                    };
                    let uto = get_uto(params[0].clone().into()).await?;

                    let satoshis: u64 = params[2].clone().into();

                    let bfs = BitcoinFilesApi::new(constants::urls::DOGEFILES_URL.to_string());
                    let contract_init = bfs.contract(&abi.contract).await?;

                    if let Some(royalty_percentage) = &contract_init.royalty_percentage {
                        let accuracy = 10u64.pow(royalty_percentage[1] + 2);
                        let percentage = u64::from(royalty_percentage[0]);
                        let royalty = satoshis * percentage / accuracy;

                        if royalty >= 1 {
                            outputs.push(TxBuilderOutput {
                                sats: royalty,
                                address: Some(contract_init.creator_address),
                                to: None,
                                script: None,
                                args: None,
                                encrypt_args: None,
                            });
                        }
                    }

                    contract.abi(brc721::basic::ABI::Purchase(
                        uto,
                        wallet.account_address()?.get_locking_script()?,
                        params[1].clone().into(),
                        satoshis,
                    ))?
                }
                SigilABIMethods::Escrow => {
                    auto_fund = false;
                    let contract = brc721::BRC721Basic {
                        store: Box::new(store),
                    };
                    let uto = get_uto(params[0].clone().into()).await?;
                    contract.abi(brc721::basic::ABI::Escrow(
                        uto,
                        params[1].clone().into(),
                        params[2].clone().into(),
                    ))?
                }
                SigilABIMethods::Vax => {
                    auto_fund = false;
                    let contract = lizervaxx::LizerVaxx {
                        store: Box::new(store),
                    };

                    let frog = get_uto(params[0].clone().into()).await?;
                    let vax = get_uto(params[1].clone().into()).await?;
                    let apu = get_uto(params[2].clone().into()).await?;

                    contract.abi(lizervaxx::ABI::Vax(frog, vax, apu)).await?
                }
                SigilABIMethods::Slurp => {
                    auto_fund = false;
                    let contract = slurp_juice::SlurpJuice {
                        store: Box::new(store),
                    };

                    let ape = get_uto(params[0].clone().into()).await?;
                    let slurp = get_uto(params[1].clone().into()).await?;

                    contract.abi(slurp_juice::ABI::Slurp(ape, slurp)).await?
                }
            };

            if let Some(mut o) = call.outputs.clone() {
                outputs.append(&mut o);
            }

            let built_tx = wallet
                .build_tx(&TxBuilder {
                    auto_fund,
                    change_address: None,
                    contract: Some(abi.contract),
                    extended_tx: None,
                    typed_signing: Some(typed_signing),
                    network: Networks::BSV,
                    outputs,
                })
                .await?;

            return Ok(TwetchPayAction {
                built_tx,
                call: call.clone(),
                is_troll_toll: None,
            });
        }

        anyhow::bail!("invalid args");
    }
}
