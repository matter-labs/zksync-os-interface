pub mod error;
pub mod primitives;

use alloy_primitives::{Address, B256, U256};

// Re-export alloy's Log struct
pub use alloy_primitives::Log;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockContext {
    // Chain id is temporarily also added here (so that it can be easily passed from the oracle)
    // long term, we have to decide whether we want to keep it here, or add a separate oracle
    // type that would return some 'chain' specific metadata (as this class is supposed to hold block metadata only).
    pub chain_id: u64,
    pub block_number: u64,
    pub block_hashes: BlockHashes,
    pub timestamp: u64,
    pub eip1559_basefee: U256,
    pub gas_per_pubdata: U256,
    pub native_price: U256,
    pub coinbase: Address,
    pub gas_limit: u64,
    pub pubdata_limit: u64,
    /// Source of randomness, currently holds the value
    /// of prevRandao.
    pub mix_hash: U256,
    /// Version of the protocol.
    /// It's used to determine the implementation of ZKsync OS and its config.
    pub protocol_version: u32,
}

/// Array of previous block hashes.
/// Hash for block number N will be at index [256 - (current_block_number - N)]
/// (most recent will be at the end) if N is one of the most recent
/// 256 blocks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockHashes(pub [U256; 256]);

impl Default for BlockHashes {
    fn default() -> Self {
        Self([U256::ZERO; 256])
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for BlockHashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_vec().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BlockHashes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<U256> = Vec::deserialize(deserializer)?;
        let array: [U256; 256] = vec
            .try_into()
            .map_err(|_| serde::de::Error::custom("Expected array of length 256"))?;
        Ok(Self(array))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreimageType {
    Bytecode = 0,
    AccountData = 1,
}

/// L2 to l1 log structure, used for merkle tree leaves.
/// This structure holds both kinds of logs (user messages
/// and l1 -> l2 tx logs).
#[derive(Default, Debug, Clone)]
pub struct L2ToL1Log {
    /// Shard id.
    /// Deprecated, kept for compatibility, always set to 0.
    pub l2_shard_id: u8,
    /// Boolean flag.
    /// Deprecated, kept for compatibility, always set to `true`.
    pub is_service: bool,
    /// The L2 transaction number in a block, in which the log was sent
    pub tx_number_in_block: u16,
    /// The L2 address which sent the log.
    /// For user messages set to `L1Messenger` system hook address,
    /// for l1 -> l2 txs logs - `BootloaderFormalAddress`.
    pub sender: Address,
    /// The 32 bytes of information that was sent in the log.
    /// For user messages used to save message sender address(padded),
    /// for l1 -> l2 txs logs - transaction hash.
    pub key: B256,
    /// The 32 bytes of information that was sent in the log.
    /// For user messages used to save message hash.
    /// for l1 -> l2 txs logs - success flag(padded).
    pub value: B256,
}

#[derive(Debug, Clone)]
pub struct L2ToL1LogWithPreimage {
    pub log: L2ToL1Log,
    pub preimage: Option<Vec<u8>>,
}
