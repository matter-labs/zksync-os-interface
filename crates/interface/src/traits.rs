use crate::error::InvalidTransaction;
use crate::tracing::AnyTracer;
use crate::types::{BlockContext, BlockOutput, TxOutput, TxProcessingOutputOwned};
use alloy_primitives::B256;
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
    Tx(Vec<u8>),
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

#[derive(Clone)]
pub struct TxListSource {
    pub transactions: VecDeque<Vec<u8>>,
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
    >(
        &self,
        config: Self::Config,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        tx_source: TrSrc,
        tx_result_callback: TrCallback,
        tracer: &mut Tracer,
    ) -> Result<BlockOutput, Self::Error>;
}

pub trait SimulateTx {
    type Config;
    type Error: fmt::Display;

    fn simulate_tx<Storage: ReadStorage, PreimgSrc: PreimageSource, Tracer: AnyTracer>(
        &self,
        config: Self::Config,
        transaction: Vec<u8>,
        block_context: BlockContext,
        storage: Storage,
        preimage_source: PreimgSrc,
        tracer: &mut Tracer,
    ) -> Result<Result<TxOutput, InvalidTransaction>, Self::Error>;
}
