#[cfg(test)]
mod polynym_tests {
    use anyhow::Result;
    use twetch_sdk::{constants, PolynymApi};

    const PAYMAIL: &str = "80515@twetch.me";

    #[tokio::test]
    async fn capabilities() -> Result<()> {
        let polynym = PolynymApi::new(constants::POLYNYM_URL.to_string());
        let c = polynym.capabilities(&PAYMAIL.to_string()).await?;

        if let Some(pki) = c.pki {
            assert_eq!(
                pki,
                "https://metasync.twetch.app/paymail/id/{alias}@{domain.tld}".to_string()
            );
        }

        if let Some(payment_destination) = c.payment_destination {
            assert_eq!(
                payment_destination,
                "https://metasync.twetch.app/paymail/payment-destination/{alias}@{domain.tld}"
                    .to_string()
            );
        }

        if let Some(verify_pubkey) = c.verify_pubkey {
            assert_eq!(
                verify_pubkey,
                "https://metasync.twetch.app/paymail/verifypubkey/{alias}@{domain.tld}/{pubkey}"
                    .to_string()
            );
        }

        if let Some(public_profile) = c.public_profile {
            assert_eq!(
                public_profile,
                "https://auth.twetch.app/api/v2/public-profile/{alias}@{domain.tld}".to_string()
            );
        }

        if let Some(p2p_receive_transaction) = c.p2p_receive_transaction {
            assert_eq!(
                p2p_receive_transaction,
                "https://metasync.twetch.app/paymail/receive-transaction/{alias}@{domain.tld}"
                    .to_string()
            );
        }
        if let Some(p2p_payment_destination) = c.p2p_payment_destination {
            assert_eq!(
                p2p_payment_destination,
                "https://metasync.twetch.app/paymail/p2p-payment-destination/{alias}@{domain.tld}"
                    .to_string()
            );
        }

        if let Some(sigil) = c.sigil {
            assert_eq!(sigil, "https://metasync.twetch.app/sigil".to_string());
        }

        Ok(())
    }

    #[tokio::test]
    async fn p2p_payment_destination() -> Result<()> {
        let polynym = PolynymApi::new(constants::POLYNYM_URL.to_string());
        let c = polynym
            .p2p_payment_destination(&PAYMAIL.to_string(), 1)
            .await?;

        assert!(c.reference.len() > 0);
        assert!(c.outputs.len() > 0);

        Ok(())
    }
}
