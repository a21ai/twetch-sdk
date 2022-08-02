#[cfg(test)]
mod wallet_tests {
    use anyhow::Result;
    use twetch_sdk::Wallet;

    const SEED: &str = "book fit fly ketchup also elevator scout mind edit fatal where rookie";

    #[tokio::test]
    async fn wallet_xpub() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.wallet_xpub()?.to_string()?,
            "xpub6Dv27qDns95XnA2rjSunSvYwaJa3z614xq79X3rMYRzpZLjnb6j6La1rvoXLLvrBWC5BABJ6pBSnkdspj94g262wRh8M4MQmEiMSQ9s7QDn".to_string()
        );
        Ok(())
    }

    #[tokio::test]
    async fn xpub() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.xpub()?.to_string()?,
            "xpub661MyMwAqRbcFYHX2uE6zQHyLdBvVdv5tddZi5nRApwmrzPpeGQb5zJFfp8jMxqv4HpYMZ8Xre7WaEfNK2612a7wTY21ASzDuoXfXHRJHXG".to_string()
        );
        Ok(())
    }

    #[tokio::test]
    async fn account_public_key() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.account_public_key()?.to_hex()?,
            "02dbe5e772d01b7bd461e3cb4960afabe7e3db6652e9eec7b70b8c10c4baf29e64".to_string()
        );
        Ok(())
    }

    #[tokio::test]
    async fn account_address() -> Result<()> {
        let wallet = Wallet::new(SEED.to_string());
        assert_eq!(
            wallet.account_address()?.to_string()?,
            "199sSk4VNnoWM4AMRUVdhPLL9jBBrRbWzN".to_string()
        );
        Ok(())
    }

    #[tokio::test]
    async fn xpub_derivation() -> Result<()> {
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
}
