use bsv_wasm::ExtendedPrivateKey;

pub struct Wallet {
    seed: String,
}

impl Wallet {
    pub fn new(seed: String) -> Wallet {
        Wallet { seed }
    }

    pub fn xpriv(&self) -> Option<ExtendedPrivateKey> {
        let xpriv = match ExtendedPrivateKey::from_mnemonic(self.seed.as_bytes(), None) {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(xpriv)
    }

    pub fn xpriv_account(&self) -> Option<ExtendedPrivateKey> {
        let xpriv = match self.xpriv() {
            Some(v) => v.derive_from_path("m/0/0").unwrap(),
            None => return None,
        };

        Some(xpriv)
    }

    pub fn xpriv_wallet(&self) -> Option<ExtendedPrivateKey> {
        let xpriv = match self.xpriv() {
            Some(v) => v.derive_from_path("m/44'/0'/0'/0").unwrap(),
            None => return None,
        };

        Some(xpriv)
    }
}
