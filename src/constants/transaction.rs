pub const TX_FEE_RATE: f64 = 0.25;
// Either 106 or 107 bytes
pub const P2PKH_UNLOCKING_SCRIPT_SIZE: usize = 107;
pub const P2PKH_OUTPUT_SIZE: usize = 34;
// Either 140 or 139 bytes
pub const SIGIL_V2_UNLOCKING_SCRIPT_SIZE: usize = 140;
pub const TX_VERSION_SIZE: usize = 4;
pub const TX_LOCKTIME_SIZE: usize = 4;

pub const TXOUT_SATOSHIS_SIZE: usize = 8;

pub const TXIN_TXID_SIZE: usize = 32;
pub const TXIN_VOUT_SIZE: usize = 4;
pub const TXIN_SEQUENCE_SIZE: usize = 4;
