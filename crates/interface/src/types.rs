use crate::error::InvalidTransaction;
use alloy::consensus::{Header, Sealed};
use alloy::primitives::{Address, B256, U256};
use serde::{Deserialize, Serialize};

pub use alloy::primitives::Log;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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

impl serde::Serialize for BlockHashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_vec().serialize(serializer)
    }
}

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

#[derive(Debug, Clone)]
pub struct TxProcessingOutputOwned {
    pub status: bool,
    pub output: Vec<u8>,
    pub contract_address: Option<Address>,
    pub gas_used: u64,
    pub gas_refunded: u64,
    pub computational_native_used: u64,
    pub native_used: u64,
    pub pubdata_used: u64,
}

#[derive(Debug, Clone)]
pub struct BlockOutput {
    pub header: Sealed<Header>,
    pub tx_results: Vec<Result<TxOutput, InvalidTransaction>>,
    // TODO: will be returned per tx later
    pub storage_writes: Vec<StorageWrite>,
    pub published_preimages: Vec<(B256, Vec<u8>, PreimageType)>,
    pub pubdata: Vec<u8>,
    pub computaional_native_used: u64,
}

#[derive(Debug, Clone)]
pub struct StorageWrite {
    pub key: B256,
    pub value: B256,
    // Additional information (account & account key).
    // hash of them is equal to the key below.
    // We export them for now, to make integration with existing systems (like anvil-zksync) easier.
    // In the future, we might want to remove these for performance reasons.
    pub account: Address,
    pub account_key: B256,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreimageType {
    Bytecode = 0,
    AccountData = 1,
}

/// Transaction output in case of successful validation.
/// This structure includes data to create receipts and update state.
#[derive(Debug, Clone)]
// Output not observed for now, we allow dead code temporarily
#[allow(dead_code)]
pub struct TxOutput {
    /// Transaction execution step result
    pub execution_result: ExecutionResult,
    /// Total gas used, including all the steps(validation, execution, postOp call)
    pub gas_used: u64,
    /// Amount of refunded gas
    pub gas_refunded: u64,
    /// Amount of native resource used in the entire transaction for computation.
    pub computational_native_used: u64,
    /// Total amount of native resource used in the entire transaction (includes spent on pubdata)
    pub native_used: u64,
    /// Amount of pubdata used in the entire transaction.
    pub pubdata_used: u64,
    /// Deployed contract address
    /// - `Some(address)` for the deployment transaction
    /// - `None` otherwise
    pub contract_address: Option<Address>,
    /// Total logs list emitted during all the steps(validation, execution, postOp call)
    pub logs: Vec<Log>,
    /// Total l2 to l1 logs list emitted during all the steps(validation, execution, postOp call)
    pub l2_to_l1_logs: Vec<L2ToL1LogWithPreimage>,
    /// Deduplicated storage writes happened during tx processing(validation, execution, postOp call)
    /// TODO: now this field empty as we return writes on the blocks level, but eventually should be moved here
    pub storage_writes: Vec<StorageWrite>,
}

impl TxOutput {
    pub fn is_success(&self) -> bool {
        matches!(self.execution_result, ExecutionResult::Success(_))
    }

    pub fn as_returned_bytes(&self) -> &[u8] {
        match &self.execution_result {
            ExecutionResult::Success(o) => match o {
                ExecutionOutput::Call(vec) => vec,
                ExecutionOutput::Create(vec, _) => vec,
            },
            ExecutionResult::Revert(vec) => vec,
        }
    }
}

///
/// L2 to l1 log structure, used for merkle tree leaves.
/// This structure holds both kinds of logs (user messages
/// and l1 -> l2 tx logs).
///
#[derive(Default, Debug, Clone)]
pub struct L2ToL1Log {
    ///
    /// Shard id.
    /// Deprecated, kept for compatibility, always set to 0.
    ///
    pub l2_shard_id: u8,
    ///
    /// Boolean flag.
    /// Deprecated, kept for compatibility, always set to `true`.
    ///
    pub is_service: bool,
    ///
    /// The L2 transaction number in a block, in which the log was sent
    ///
    pub tx_number_in_block: u16,
    ///
    /// The L2 address which sent the log.
    /// For user messages set to `L1Messenger` system hook address,
    /// for l1 -> l2 txs logs - `BootloaderFormalAddress`.
    ///
    pub sender: Address,
    ///
    /// The 32 bytes of information that was sent in the log.
    /// For user messages used to save message sender address(padded),
    /// for l1 -> l2 txs logs - transaction hash.
    ///
    pub key: B256,
    ///
    /// The 32 bytes of information that was sent in the log.
    /// For user messages used to save message hash.
    /// for l1 -> l2 txs logs - success flag(padded).
    ///
    pub value: B256,
}

#[derive(Debug, Clone)]
pub struct L2ToL1LogWithPreimage {
    pub log: L2ToL1Log,
    pub preimage: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
// Output not observed for now, we allow dead code temporarily
#[allow(dead_code)]
pub enum ExecutionOutput {
    Call(Vec<u8>),
    Create(Vec<u8>, Address),
}

#[derive(Debug, Clone)]
// Output not observed for now, we allow dead code temporarily
#[allow(dead_code)]
pub enum ExecutionResult {
    /// Transaction executed successfully
    Success(ExecutionOutput),
    /// Transaction reverted
    Revert(Vec<u8>),
}
