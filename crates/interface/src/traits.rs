use crate::error::InvalidTransaction;
use crate::tracing::{AnyTracer, AnyTxValidator};
use crate::types::{TxOutput, TxProcessingOutputOwned};
use alloy_primitives::{Address, B256, U256, hex};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;
use std::ops::DerefMut;

pub trait ReadStorage: 'static {
    fn read(&mut self, key: B256) -> Option<B256>;
}

impl<T: ReadStorage> ReadStorage for Box<T> {
    fn read(&mut self, key: B256) -> Option<B256> {
        self.deref_mut().read(key)
    }
}

pub trait PreimageSource: 'static {
    fn get_preimage(&mut self, hash: B256) -> Option<Vec<u8>>;
}

impl<T: PreimageSource> PreimageSource for Box<T> {
    fn get_preimage(&mut self, hash: B256) -> Option<Vec<u8>> {
        self.deref_mut().get_preimage(hash)
    }
}

#[derive(Debug, Clone)]
pub enum NextTxResponse {
    Tx(EncodedTx),
    SealBlock,
}

pub trait TxSource: 'static {
    fn get_next_tx(&mut self) -> NextTxResponse;
}

pub trait TxResultCallback: 'static {
    fn tx_executed(
        &mut self,
        tx_execution_result: Result<TxProcessingOutputOwned, InvalidTransaction>,
    );
}

/// Source of raw FRI proof bytes keyed by `statement_versioned_hash`.
///
/// Used by the bootloader's FRI oracle responder to resolve
/// `FRI_PROOF_QUERY_ID` queries during gateway block execution.
pub trait FriProofSidecarSource: 'static {
    /// Returns the raw (bincode-serialized) `UnrolledProgramProof`
    /// bytes stored under this `statement_versioned_hash`.
    ///
    /// Returns `None` if the sidecar has no entry for this hash.
    fn get_proof_bytes(&mut self, statement_versioned_hash: B256) -> Option<Vec<u8>>;
}

/// No-op sidecar source used when FRI proof verification is not
/// wired up (non-gateway chains, eth_call, ETH-replay, etc.).
#[derive(Debug, Clone, Copy, Default)]
pub struct NoFriProofSidecar;

impl FriProofSidecarSource for NoFriProofSidecar {
    fn get_proof_bytes(&mut self, _statement_versioned_hash: B256) -> Option<Vec<u8>> {
        None
    }
}

/// Source of the L1 read witness for the L1SLOAD precompile.
///
/// The precompile announces each read before verifying it, and verifies everything it is served
/// against the anchored L1 block hash. A source can make a read fail, but it can't make the OS
/// accept wrong L1 state.
///
/// Sequencers fetch from L1 and record what they served; replays serve the recorded witness.
pub trait L1WitnessSource: 'static {
    /// Returns the keccak256 preimages that prove `keys` of `address` at the L1 block whose hash is
    /// `anchor`, i.e. an `eth_getProof` request at that block: the anchor's block header and the
    /// account and storage proof nodes.
    ///
    /// Called synchronously during execution, once per announced read, with no deduplication across
    /// reads. A missing preimage rejects the transaction with
    /// [`InvalidTransaction::L1WitnessMissing`], so an empty result means "not available".
    ///
    /// Within its size caps, the OS keeps every preimage served during a block and may use it for any
    /// later read in that block. Proof nodes are kept for the whole block, and a block header only
    /// under the anchor it hashes to. So a source that records the block's witness must record every
    /// preimage it returns, including those served to transactions that end up rejected, and a
    /// replaying source must serve each anchor's header for a read at that anchor.
    fn fetch(&mut self, anchor: B256, address: Address, keys: &[B256]) -> Vec<Vec<u8>>;
}

impl<T: L1WitnessSource> L1WitnessSource for Box<T> {
    fn fetch(&mut self, anchor: B256, address: Address, keys: &[B256]) -> Vec<Vec<u8>> {
        self.deref_mut().fetch(anchor, address, keys)
    }
}

/// Source with no L1 witness, for chains without L1SLOAD and paths that never read L1.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoL1Witness;

impl L1WitnessSource for NoL1Witness {
    fn fetch(&mut self, _anchor: B256, _address: Address, _keys: &[B256]) -> Vec<Vec<u8>> {
        Vec::new()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum EncodedTx {
    Abi(Vec<u8>),
    Rlp(Vec<u8>, Address),
}

impl fmt::Debug for EncodedTx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abi(bytes) => f
                .debug_tuple("Abi")
                .field(&format!("0x{}", hex::encode(bytes)))
                .finish(),
            Self::Rlp(bytes, addr) => f
                .debug_tuple("Rlp")
                .field(&format!("0x{}", hex::encode(bytes)))
                .field(&format_args!("signer: {}", addr))
                .finish(),
        }
    }
}

impl EncodedTx {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            Self::Abi(tx) | Self::Rlp(tx, _) => tx.len(),
        }
    }

    pub fn bytes(&self) -> &Vec<u8> {
        match self {
            Self::Abi(tx) | Self::Rlp(tx, _) => tx,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TxListSource {
    pub transactions: VecDeque<EncodedTx>,
}

impl TxSource for TxListSource {
    fn get_next_tx(&mut self) -> NextTxResponse {
        match self.transactions.pop_front() {
            Some(tx) => NextTxResponse::Tx(tx),
            None => NextTxResponse::SealBlock,
        }
    }
}

#[derive(Clone)]
pub struct NoopTxCallback;

impl TxResultCallback for NoopTxCallback {
    fn tx_executed(
        &mut self,
        _tx_execution_result: Result<TxProcessingOutputOwned, InvalidTransaction>,
    ) {
    }
}

pub trait AnyBlockContext {
    fn chain_id(&self) -> u64;
    fn block_number(&self) -> u64;
    fn block_hashes(&self) -> &[U256; 256];
    fn timestamp(&self) -> u64;
    fn eip1559_basefee(&self) -> U256;
    fn pubdata_price(&self) -> U256;
    fn native_price(&self) -> U256;
    fn coinbase(&self) -> Address;
    fn gas_limit(&self) -> u64;
    fn pubdata_limit(&self) -> u64;
    /// Source of randomness, currently holds the value of prevRandao.
    fn mix_hash(&self) -> U256;
    fn blob_fee(&self) -> U256;
    /// Whether this block is executed on a Gateway chain.
    /// Gateway chains support additional features such as FRI proof verification.
    fn is_gateway(&self) -> bool;
}

pub trait RunBlock {
    type Config;
    type Error: fmt::Display;
    type BlockOutput;

    #[allow(clippy::too_many_arguments)]
    fn run_block<
        Storage: ReadStorage,
        PreimgSrc: PreimageSource,
        TrSrc: TxSource,
        FriSidecar: FriProofSidecarSource,
        TrCallback: TxResultCallback,
        Tracer: AnyTracer,
        Valdiator: AnyTxValidator,
        BlockContext: AnyBlockContext,
    >(
        &self,
        config: Self::Config,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        tx_source: TrSrc,
        fri_proof_sidecar: FriSidecar,
        tx_result_callback: TrCallback,
        tracer: &mut Tracer,
        validator: &mut Valdiator,
    ) -> Result<Self::BlockOutput, Self::Error>;

    /// [`RunBlock::run_block`] with an [`L1WitnessSource`] for the L1SLOAD precompile.
    ///
    /// The default ignores the source and runs [`RunBlock::run_block`], so callers can pass a source
    /// to versions without L1SLOAD. Versions with L1SLOAD must override it, and an implementation that
    /// doesn't override it must not implement `run_block` by calling it: the two would recurse.
    #[allow(clippy::too_many_arguments)]
    fn run_block_with_l1_witness<
        Storage: ReadStorage,
        PreimgSrc: PreimageSource,
        TrSrc: TxSource,
        FriSidecar: FriProofSidecarSource,
        TrCallback: TxResultCallback,
        L1WitnessSrc: L1WitnessSource,
        Tracer: AnyTracer,
        Validator: AnyTxValidator,
        BlockContext: AnyBlockContext,
    >(
        &self,
        config: Self::Config,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        tx_source: TrSrc,
        fri_proof_sidecar: FriSidecar,
        tx_result_callback: TrCallback,
        _l1_witness: L1WitnessSrc,
        tracer: &mut Tracer,
        validator: &mut Validator,
    ) -> Result<Self::BlockOutput, Self::Error> {
        self.run_block(
            config,
            block_context,
            storage,
            preimage_source,
            tx_source,
            fri_proof_sidecar,
            tx_result_callback,
            tracer,
            validator,
        )
    }
}

pub trait SimulateTx {
    type Config;
    type Error: fmt::Display;

    #[allow(clippy::too_many_arguments)]
    fn simulate_tx<
        Storage: ReadStorage,
        PreimgSrc: PreimageSource,
        Tracer: AnyTracer,
        Validator: AnyTxValidator,
        BlockContext: AnyBlockContext,
    >(
        &self,
        config: Self::Config,
        transaction: EncodedTx,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        tracer: &mut Tracer,
        validator: &mut Validator,
    ) -> Result<Result<TxOutput, InvalidTransaction>, Self::Error>;

    /// [`SimulateTx::simulate_tx`] with an [`L1WitnessSource`] for the L1SLOAD precompile.
    ///
    /// The same contract as [`RunBlock::run_block_with_l1_witness`]: the default ignores the source,
    /// versions with L1SLOAD must override it, and `simulate_tx` must not call it unless it is
    /// overridden.
    #[allow(clippy::too_many_arguments)]
    fn simulate_tx_with_l1_witness<
        Storage: ReadStorage,
        PreimgSrc: PreimageSource,
        L1WitnessSrc: L1WitnessSource,
        Tracer: AnyTracer,
        Validator: AnyTxValidator,
        BlockContext: AnyBlockContext,
    >(
        &self,
        config: Self::Config,
        transaction: EncodedTx,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        _l1_witness: L1WitnessSrc,
        tracer: &mut Tracer,
        validator: &mut Validator,
    ) -> Result<Result<TxOutput, InvalidTransaction>, Self::Error> {
        self.simulate_tx(
            config,
            transaction,
            block_context,
            storage,
            preimage_source,
            tracer,
            validator,
        )
    }
}
