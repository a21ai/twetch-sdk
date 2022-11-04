use crate::{constants, GraphqlApi, Message, Wallet};
use anyhow::Result;
use base64;
use bsv::{ECIESCiphertext, Hash, PrivateKey, PublicKey, ECIES};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    pub id: String,
    pub key: String,
}

impl Conversation {
    pub async fn create(token: String, user_ids: Vec<String>) -> Result<Conversation> {
        let api = GraphqlApi::new(constants::GATEWAY_URL.to_string(), token);

        let pubkeys = api
            .list_pubkeys(user_ids.clone())
            .await?
            .as_array()
            .unwrap()
            .clone();

        let key = Conversation::generate_key();

        let mut conversation_users = Vec::new();

        for e in 0..user_ids.len() {
            let pubkey = pubkeys[e]
                .get("publicKey")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();

            let user_id = pubkeys[e].get("id").unwrap().as_str().unwrap().to_string();

            conversation_users.push(json!({
                "userId": user_id,
                "encryptedKey": Conversation::encrypt_key(key.clone(), pubkey).unwrap()
            }))
        }

        let payload =
            json!({ "conversationUsers": serde_json::to_value(conversation_users).unwrap() })
                .to_string();

        let res = api.create_conversation(payload).await?;

        Ok(Conversation {
            id: res.get("id").unwrap().as_str().unwrap().to_string(),
            key,
        })
    }

    pub async fn set_name(token: String, conversation: String, name: String) -> Result<()> {
        let api = GraphqlApi::new(constants::GATEWAY_URL.to_string(), token);
        api.update_conversation(conversation, name).await?;
        Ok(())
    }

    pub async fn create_message(&self, token: String, description: String) -> Result<Message> {
        Message::create(
            self.key.clone(),
            self.id.clone(),
            "1".to_string(),
            description,
            token,
        )
        .await
    }

    pub fn generate_key() -> String {
        Hash::sha_256(&PrivateKey::from_random().to_bytes())
            .to_hex()
            .chars()
            .skip(32)
            .take(32)
            .collect()
    }

    pub fn encrypt_key(key: String, pubkey: String) -> Result<String> {
        let public_key = PublicKey::from_hex(&pubkey)?;
        let encrypted =
            ECIES::encrypt_with_ephemeral_private_key(key.as_bytes(), &public_key)?.to_bytes();

        Ok(base64::encode_config(encrypted, base64::STANDARD))
    }

    pub fn decrypt_key(encrypted_key: String, seed: String) -> Result<Vec<u8>> {
        let wallet = Wallet::new(seed);
        let private_key = wallet.account_private_key()?;
        let encrypted_key_buf = base64::decode_config(encrypted_key, base64::STANDARD)?;
        let ciphertext = ECIESCiphertext::from_bytes(&encrypted_key_buf, true)?;
        let pubkey = ciphertext.extract_public_key()?;
        let decrypted = ECIES::decrypt(&ciphertext, &private_key, &pubkey)?;
        Ok(decrypted)
    }
}
