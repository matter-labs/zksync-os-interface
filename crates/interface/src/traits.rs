use crate::error::InvalidTransaction;
use crate::tracing::{AnyTracer, AnyTxValidator};
use crate::types::{BlockContext, BlockOutput, TxOutput, TxProcessingOutputOwned};
use alloy_primitives::{Address, B256};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;

pub trait ReadStorage: 'static {
    fn read(&mut self, key: B256) -> Option<B256>;
}

pub trait PreimageSource: 'static {
    fn get_preimage(&mut self, hash: B256) -> Option<Vec<u8>>;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncodedTx {
    Abi(Vec<u8>),
    Rlp(Vec<u8>, Address),
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

pub trait RunBlock {
    type Config;
    type Error: fmt::Display;

    #[allow(clippy::too_many_arguments)]
    fn run_block<
        Storage: ReadStorage,
        PreimgSrc: PreimageSource,
        TrSrc: TxSource,
        TrCallback: TxResultCallback,
        Tracer: AnyTracer,
        Valdiator: AnyTxValidator,
    >(
        &self,
        config: Self::Config,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        tx_source: TrSrc,
        tx_result_callback: TrCallback,
        tracer: &mut Tracer,
        validator: &mut Valdiator,
    ) -> Result<BlockOutput, Self::Error>;
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
}
