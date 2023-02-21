#[cfg(test)]
mod wallet_tests {
    use anyhow::Result;
    use bsv::{ExtendedPublicKey, Script, TxOut};
    use twetch_sdk::{Networks, TxBuilder, TxBuilderOutput, Wallet};

    const SEED: &str = "book fit fly ketchup also elevator scout mind edit fatal where rookie";

    #[test]
    fn nft() -> Result<()> {
        let hex = "a914a4f6cbc6645044112d3b45ca50b4af40d35c87598876a91414a8036c8b3d910a7e24d46067048d8761274b5588ac6a4d3a027b227469746c65223a224c697a657256415858e284a2202333363033222c226465736372697074696f6e223a224c697a657256415858e284a220697320612053414645e284a220616e6420454646454354495645e284a22076616363696e6520646576656c6f706564206279204c697a6572436f7270e284a220746f206669676874204150552d31392e222c22696d616765223a22623a2f2f31613031383162613664633661353562303237616334663435653831353761656237333935323662396631376166306437613863306165663234616635633031222c226e756d626572223a333630332c22736572696573223a333639302c2261747472696275746573223a5b7b2274726169745f74797065223a2254726169742031222c2276616c7565223a2253616665e284a2222c22636f756e74223a333639302c22726172697479223a22436f6d6d6f6e227d2c7b2274726169745f74797065223a2254726169742032222c2276616c7565223a22456666656374697665e284a2222c22636f756e74223a333639302c22726172697479223a22436f6d6d6f6e227d2c7b2274726169745f74797065223a2254726169742033222c2276616c7565223a2231737420446f7365e284a2222c22636f756e74223a333639302c22726172697479223a22436f6d6d6f6e227d5d2c22676966223a22623a2f2f66316561323464386564326534663933313331393664353164393862643133343536653639336232643733373432626234393364343762316238383035613038227d";

        let tx_out = TxOut::new(0, &Script::from_hex(hex)?);

        let nft = TxBuilder::find_nft(tx_out);

        assert_eq!(nft.unwrap().title, "LizerVAXX™ #3603");
        Ok(())
    }

    #[test]
    fn wallet_xpub() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.wallet_xpub()?.to_string()?,
            "xpub6Dv27qDns95XnA2rjSunSvYwaJa3z614xq79X3rMYRzpZLjnb6j6La1rvoXLLvrBWC5BABJ6pBSnkdspj94g262wRh8M4MQmEiMSQ9s7QDn".to_string()
        );
        Ok(())
    }

    #[test]
    fn xpub() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.xpub()?.to_string()?,
            "xpub661MyMwAqRbcFYHX2uE6zQHyLdBvVdv5tddZi5nRApwmrzPpeGQb5zJFfp8jMxqv4HpYMZ8Xre7WaEfNK2612a7wTY21ASzDuoXfXHRJHXG".to_string()
        );
        Ok(())
    }

    #[test]
    fn account_public_key() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.account_public_key()?.to_hex()?,
            "02dbe5e772d01b7bd461e3cb4960afabe7e3db6652e9eec7b70b8c10c4baf29e64".to_string()
        );
        Ok(())
    }

    #[test]
    fn account_address() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.account_address()?.to_string()?,
            "199sSk4VNnoWM4AMRUVdhPLL9jBBrRbWzN".to_string()
        );
        Ok(())
    }

    #[test]
    fn display_addresses() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.display_address(&Networks::BSV)?.to_string(),
            "199sSk4VNnoWM4AMRUVdhPLL9jBBrRbWzN".to_string()
        );
        assert_eq!(
            wallet.display_address(&Networks::TBSV)?.to_string(),
            "mofpjo9UBpEm8Ady93U1XJYf1imtp7Qbin".to_string()
        );
        Ok(())
    }

    #[test]
    fn xpub_derivation() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());

        let xpub = wallet.xpub()?.derive_from_path("m/0/0")?;

        assert_eq!(
            xpub.get_public_key().to_hex()?,
            wallet.account_public_key()?.to_hex()?
        );

        assert_eq!(
            xpub.get_public_key().to_p2pkh_address()?.to_string()?,
            wallet.account_address()?.to_string()?
        );

        Ok(())
    }

    #[test]
    fn xpub_xpriv_match() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());

        assert_eq!(
            wallet.wallet_xpub()?.get_public_key().to_hex()?,
            ExtendedPublicKey::from_xpriv(&wallet.wallet_xpriv()?)
                .get_public_key()
                .to_hex()?
        );

        Ok(())
    }

    //#[test]
    //fn segwit_address() -> Result<()> {
    //let wallet = Wallet::new(SEED.to_string());
    //let segwit_address = wallet.display_address_segwit()?;

    //assert_eq!(
    //segwit_address,
    //"bc1qksjasa8m2zk8ram3mv8ne4w8skjg9ywelxv84h".to_string()
    //);

    //Ok(())
    //}

    //#[tokio::test]
    //async fn tx_builder() -> Result<()> {
    //let wallet = Wallet::new(SEED.to_string());

    //let builder = TxBuilder {
    //network: Networks::BSV,
    //contract: None,
    //extended_tx: None,
    //typed_signing: None,
    //outputs: vec![
    //TxBuilderOutput {
    //sats: 100,
    //address: None,
    //to: Some("@1".to_string()),
    //args: None,
    //encrypt_args: None,
    //script: None,
    //},
    //TxBuilderOutput {
    //sats: 100,
    //address: None,
    //to: Some("1".to_string()),
    //args: None,
    //encrypt_args: None,
    //script: None,
    //},
    //TxBuilderOutput {
    //sats: 100,
    //address: None,
    //to: Some("12tDncQvFZaZzqanupmtXpDUm42Wd4Cn4W".to_string()),
    //args: None,
    //encrypt_args: None,
    //script: None,
    //},
    //TxBuilderOutput {
    //sats: 100,
    //address: None,
    //to: Some("harry@relayx.io".to_string()),
    //args: None,
    //encrypt_args: None,
    //script: None,
    //},
    //TxBuilderOutput {
    //sats: 100,
    //address: None,
    //to: Some("type@handcash.io".to_string()),
    //args: None,
    //encrypt_args: None,
    //script: None,
    //},
    //],
    //change_address: None,
    //auto_fund: false,
    //};

    //let built = wallet.build_tx(&builder).await?;
    //let tx = built.tx;

    //assert_eq!(
    //tx.get_output(0).unwrap().get_script_pub_key_hex(),
    //"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
    //);
    //assert_eq!(
    //tx.get_output(1).unwrap().get_script_pub_key_hex(),
    //"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
    //);
    //assert_eq!(
    //tx.get_output(2).unwrap().get_script_pub_key_hex(),
    //"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
    //);
    //assert_eq!(
    //tx.get_output(3).unwrap().get_script_pub_key_hex(),
    //"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
    //);
    //assert_eq!(tx.get_output(4).unwrap().get_script_pub_key_hex().len(), 50);
    //assert_eq!(built.payment_destinations.len(), 1);

    //Ok(())
    //}
}
