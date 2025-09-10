use zksync_os_types::error::InvalidTransaction;
use alloy_primitives::{Address, B256, U256};
use alloy_consensus::{Header, Sealed};
use zksync_os_types::{L2ToL1LogWithPreimage, PreimageType};
use alloc::vec::Vec;

// Re-export alloy's Log
pub use alloy_primitives::Log;

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
    pub account_diffs: Vec<AccountDiff>,
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

#[derive(Debug, Clone)]
pub struct AccountDiff {
    pub address: Address,
    pub nonce: u64,
    pub balance: U256,
    pub bytecode_hash: B256,
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
