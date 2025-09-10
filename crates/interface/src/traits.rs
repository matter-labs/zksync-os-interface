use crate::output::{BlockOutput, TxOutput, TxProcessingOutputOwned};
use alloc::fmt;
use alloc::vec::Vec;
use alloy_primitives::B256;
use zksync_os_types::BlockContext;
use zksync_os_types::error::InvalidTransaction;

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
    type Config;
    type Error: fmt::Display;

    fn run_block<T: ReadStorage, PS: PreimageSource, TS: TxSource, TR: TxResultCallback>(
        &self,
        config: Self::Config,
        block_context: BlockContext,
        storage: T,
        preimage_source: PS,
        tx_source: TS,
        tx_result_callback: TR,
    ) -> Result<BlockOutput, Self::Error>;
}

pub trait SimulateTx {
    type Config;
    type Error: fmt::Display;

    fn simulate_tx<S: ReadStorage, PS: PreimageSource>(
        &self,
        config: Self::Config,
        transaction: Vec<u8>,
        block_context: BlockContext,
        storage: S,
        preimage_source: PS,
    ) -> Result<Result<TxOutput, InvalidTransaction>, Self::Error>;
}
