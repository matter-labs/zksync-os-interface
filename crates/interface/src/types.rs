use alloy_primitives::{Address, B256, U256};

// Re-export alloy's Log
pub use alloy_primitives::Log;

/// Where, if at all, the operator's [`TxValidator`](crate::tracing::TxValidator)
/// rejected a priority (L1 -> L2) transaction on a chain with L1 transaction
/// filtering enabled.
///
/// A rejected priority transaction is not dropped from the block (priority
/// operations must be processed and proven); it is force-reverted with its full
/// gas limit charged. The sequencer must hand the decisions of a block, in
/// order, to witness generation so the proving run reverts at the same point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L1TxOperatorDecision {
    /// Not rejected.
    Accepted,
    /// Rejected by `begin_tx`: the body was not executed.
    RejectedBeforeExecution,
    /// Rejected by `finish_tx`: the executed body was rolled back.
    RejectedAfterExecution,
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
    /// For priority (L1 -> L2) transactions: the operator's decision
    /// (`Accepted` unless the chain filters L1 transactions). `None` for every
    /// other transaction kind.
    pub l1_tx_operator_decision: Option<L1TxOperatorDecision>,
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
    /// For priority (L1 -> L2) transactions: the operator's decision
    /// (`Accepted` unless the chain filters L1 transactions). `None` for every
    /// other transaction kind.
    pub l1_tx_operator_decision: Option<L1TxOperatorDecision>,
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
