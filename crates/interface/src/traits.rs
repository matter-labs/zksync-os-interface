use crate::error::InvalidTransaction;
use crate::types::{BlockContext, BlockOutput, TxProcessingOutputOwned};
use alloy::primitives::B256;

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

pub trait RunBlock {
    type Error: std::fmt::Display;

    fn run_block<T: ReadStorage, PS: PreimageSource, TS: TxSource, TR: TxResultCallback>(
        block_context: BlockContext,
        storage: T,
        preimage_source: PS,
        tx_source: TS,
        tx_result_callback: TR,
    ) -> Result<BlockOutput, Self::Error>;
}
