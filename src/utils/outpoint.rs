use anyhow::Result;

pub struct Outpoint {}

impl Outpoint {
    pub fn encode(txid: String, vout: u32) -> Result<String> {
        let mut buf = Vec::new();
        let mut txid = hex::decode(txid)?;
        txid.reverse();

        buf.append(&mut txid);
        buf.append(&mut vout.to_le_bytes().to_vec());

        Ok(hex::encode(buf))
    }

    pub fn decode(outpoint: String) -> Result<(String, u32)> {
        let data = hex::decode(outpoint)?;

        let mut txid = (&data[0..32]).to_vec();
        txid.reverse();
        let vout = u32::from_le_bytes([data[32], data[33], data[34], data[35]]);

        Ok((hex::encode(txid), vout))
    }
}
