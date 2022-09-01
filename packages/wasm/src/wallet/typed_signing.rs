use bsv_wasm::SigHash;
use sigil_types::{
    SigningType as RSigningType, TypedSignature as RTypedSignature, TypedSigning as RTypedSigning,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TypedSigning(RTypedSigning);

impl From<RTypedSigning> for TypedSigning {
    fn from(v: RTypedSigning) -> TypedSigning {
        TypedSigning(v)
    }
}

impl From<TypedSigning> for RTypedSigning {
    fn from(v: TypedSigning) -> RTypedSigning {
        v.0
    }
}

#[wasm_bindgen]
pub enum SigningType {
    Raw,
    Message,
    SigHash,
    SigHashR,
}

impl From<SigningType> for RSigningType {
    fn from(v: SigningType) -> RSigningType {
        match v {
            SigningType::Raw => RSigningType::Raw,
            SigningType::Message => RSigningType::Message,
            SigningType::SigHash => RSigningType::SigHash,
            SigningType::SigHashR => RSigningType::SigHashR,
        }
    }
}

impl From<RSigningType> for SigningType {
    fn from(v: RSigningType) -> SigningType {
        match v {
            RSigningType::Raw => SigningType::Raw,
            RSigningType::Message => SigningType::Message,
            RSigningType::SigHash => SigningType::SigHash,
            RSigningType::SigHashR => SigningType::SigHashR,
        }
    }
}

#[wasm_bindgen]
impl TypedSigning {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.0.data.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn num_signatures(&self) -> usize {
        self.0.signatures.len()
    }

    pub fn get_signature(&self, index: usize) -> TypedSignature {
        self.0.signatures[index].clone().into()
    }
}

#[wasm_bindgen]
pub struct TypedSignature(RTypedSignature);

impl From<RTypedSignature> for TypedSignature {
    fn from(v: RTypedSignature) -> TypedSignature {
        TypedSignature(v)
    }
}

impl From<TypedSignature> for RTypedSignature {
    fn from(v: TypedSignature) -> RTypedSignature {
        v.0
    }
}

#[wasm_bindgen]
impl TypedSignature {
    #[wasm_bindgen(getter)]
    pub fn signing_type(&self) -> SigningType {
        self.0.signing_type.into()
    }

    #[wasm_bindgen(getter)]
    pub fn sighash(&self) -> Option<SigHash> {
        match self.0.sighash {
            Some(v) => Some(v.into()),
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> Option<Vec<u8>> {
        self.0.signature.clone()
    }
}
