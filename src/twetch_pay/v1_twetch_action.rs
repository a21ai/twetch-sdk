use crate::{
    constants, ABIv1, Api, MetasyncApi, Networks, PayParams, PaymentDestination, PolynymApi,
    PublishParams, TwetchPayAction, TwetchPayCall, TxBuilder, TxBuilderOutput, Wallet,
};
use anyhow::Result;
use bsv::{P2PKHAddress, Transaction};

pub struct V1TwetchAction {}
impl V1TwetchAction {
    pub async fn run(wallet: &Wallet, call: &TwetchPayCall) -> Result<TwetchPayAction> {
        let mut outputs: Vec<TxBuilderOutput> = Vec::new();
        let mut change_address: Option<P2PKHAddress> = None;
        let mut is_troll_toll: bool = false;

        if let Some(action) = &call.action {
            if let Some(args) = &call.args {
                let mut abi = ABIv1::from_object(action, args)?;
                let api = Api::new(
                    constants::API_URL.to_string(),
                    (&wallet.token.clone().unwrap()).to_string(),
                );

                let response = api.payees(action, &abi.args).await?;

                response.payees.iter().for_each(|e| {
                    if let Some(types) = &e.types {
                        match types
                            .iter()
                            .find(|t| t.to_string() == "troll-toll".to_string())
                        {
                            Some(_) => is_troll_toll = true,
                            None => {}
                        };
                    }
                });

                abi.replace("#{invoice}".to_string(), response.invoice)?;

                let signature = wallet.sign_message(abi.content_hash()?)?;

                abi.replace("#{mySignature}".to_string(), signature)?;
                abi.replace(
                    "#{myAddress}".to_string(),
                    wallet.account_address()?.to_string()?,
                )?;

                outputs.push(TxBuilderOutput {
                    sats: 0,
                    address: None,
                    to: None,
                    args: Some(abi.args.clone()),
                    encrypt_args: call.encrypt_args,
                });

                for payee in &response.payees {
                    if let Some(sats) = payee.sats {
                        outputs.push(TxBuilderOutput {
                            sats: sats,
                            address: Some(payee.to.clone()),
                            to: None,
                            args: None,
                            encrypt_args: None,
                        })
                    }

                    if payee.amount.is_string() && payee.amount.as_str().unwrap() == "change" {
                        change_address = Some(P2PKHAddress::from_string(&payee.to)?);
                    }
                }
            }
        }

        if let Some(call_outputs) = call.outputs.clone() {
            outputs.append(&mut call_outputs.clone());
        }

        let built_tx = wallet
            .build_tx(&TxBuilder {
                auto_fund: true,
                change_address,
                contract: None,
                extended_tx: None,
                typed_signing: None,
                network: call.network.clone(),
                outputs,
            })
            .await?;

        Ok(TwetchPayAction {
            built_tx,
            call: call.clone(),
            is_troll_toll: Some(is_troll_toll),
        })
    }

    pub async fn submit_payment_destination(
        tx: &Transaction,
        payment_destination: &PaymentDestination,
        wallet: &Wallet,
    ) -> Result<()> {
        let api = PolynymApi::new(constants::POLYNYM_URL.to_string());
        api.submit_p2p_payment(tx, payment_destination, wallet)
            .await?;
        Ok(())
    }

    pub async fn submit(wallet: &Wallet, action: &TwetchPayAction) -> Result<PublishParams> {
        let mut publish_params = PublishParams { token: None };

        for e in &action.built_tx.payment_destinations {
            match V1TwetchAction::submit_payment_destination(&action.built_tx.tx, e, wallet).await {
                Ok(_) => {}
                Err(_) => {}
            };
        }

        if let Some(action_name) = &action.call.action {
            let api = Api::new(
                constants::API_URL.to_string(),
                (&wallet.token.clone().unwrap()).to_string(),
            );

            let mut pay_params = action.call.pay_params.clone();

            if let Some(params) = pay_params.clone() {
                pay_params = Some(params.clone());
            }

            if let Some(encrypted_hash) = &action.built_tx.encrypted_hash {
                if let Some(mut params) = pay_params.clone() {
                    params.encrypted_hash = Some(encrypted_hash.to_string());
                    pay_params = Some(params.clone());
                } else {
                    pay_params = Some(PayParams {
                        tweet_from_twetch: None,
                        encrypted_hash: Some(encrypted_hash.to_string()),
                        files_encrypted_hashes: None,
                    });
                }
            }

            let response = api
                .publish(action_name, &action.built_tx.tx, pay_params.clone())
                .await?;

            if let Some(errors) = response.errors {
                if errors.len() > 0 {
                    anyhow::bail!(format!("Publish Error: {}", errors.join(" ")))
                }
            }

            match response.publish_params {
                Some(v) => {
                    publish_params = v;
                }
                None => {}
            }
        } else {
            let api = MetasyncApi::new(constants::METASYNC_URL.to_string());
            api.broadcast(&action.built_tx.tx, &action.call.network, wallet)
                .await?;
        }

        Ok(publish_params)
    }
}
