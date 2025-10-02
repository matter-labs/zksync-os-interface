use alloy_primitives::{Address, B256, U256};
use zksync_os_evm_errors::EvmError;

pub trait EvmTracer {
    /// Hook immediately before external call or deployment frame execution
    fn on_new_execution_frame(&mut self, request: impl EvmRequest);

    /// Hook immediately after external call or deployment frame execution
    ///
    /// Note: `result` is None if execution is terminated due to internal ZKsync OS error (e.g. out-of-native-resources)
    fn after_execution_frame_completed(&mut self, result: Option<(EvmResources, CallResult)>);

    /// Is called on storage read produced by bytecode execution in EVM
    fn on_storage_read(&mut self, is_transient: bool, address: Address, key: B256, value: B256);

    /// Is called on storage read produced by bytecode execution in EVM
    fn on_storage_write(&mut self, is_transient: bool, address: Address, key: B256, value: B256);

    /// Is called on a change of bytecode for some account.
    /// `new_raw_bytecode` can be None if bytecode is unknown at the moment of change (e.g. force deploy by hash in system hook)
    ///
    /// Note: currently is *not* triggered by system hooks
    fn on_bytecode_change(
        &mut self,
        address: Address,
        new_raw_bytecode: Option<&[u8]>,
        new_internal_bytecode_hash: B256,
        new_observable_bytecode_length: u32,
    );

    /// Is called before EVM emits any event
    fn on_event(&mut self, address: Address, topics: Vec<B256>, data: &[u8]);

    /// Is called before bootloader starts execution of a transaction
    fn begin_tx(&mut self, calldata: &[u8]);

    /// Is called after bootloader finishes execution of a transaction
    fn finish_tx(&mut self);

    /// Called before opcode execution
    /// EE provides an access to EVM frame state, but it is not possible to read global state (storage etc) now
    fn before_evm_interpreter_execution_step(
        &mut self,
        opcode: u8,
        frame_state: impl EvmFrameInterface,
    );

    /// Called after opcode execution
    /// EE provides an access to EVM frame state, but it is not possible to read global state (storage etc) now
    ///
    /// Note: for Create/Call opcodes this hook is called BEFORE new execution frame is created.
    /// Due to current design, EVM frame state can be changed after this hook (because of charging for reading callee's account properties).
    fn after_evm_interpreter_execution_step(
        &mut self,
        opcode: u8,
        frame_state: impl EvmFrameInterface,
    );

    /// Called if some failure happens during opcode execution
    fn on_opcode_error(&mut self, error: &EvmError, frame_state: impl EvmFrameInterface);

    /// Called if some call-specific failure happened
    /// Note: unfortunately we can't provide frame state here by design (frame technically doesn't exist yet)
    fn on_call_error(&mut self, error: &EvmError);

    /// Called during selfdestruct execution
    fn on_selfdestruct(
        &mut self,
        beneficiary: Address,
        token_value: U256,
        frame_state: impl EvmFrameInterface,
    );

    /// Called on CREATE/CREATE2 system request.
    /// Hook is called before new execution frame is created.
    /// Note: CREATE/CREATE2 opcode execution can fail after this hook (and call on_opcode_error correspondingly)
    /// Note: top-level deployment won't trigger this hook
    fn on_create_request(&mut self, is_create2: bool);
}

pub trait EvmRequest {
    /// Resources left
    fn resources(&self) -> EvmResources;
    /// Caller address
    fn caller(&self) -> Address;
    /// Callee address
    fn callee(&self) -> Address;
    fn modifier(&self) -> CallModifier;
    fn input(&self) -> &[u8];
    /// Base tokens attached to this call.
    fn nominal_token_value(&self) -> U256;
}

/// Expected interface of and EVM frame state. This trait simplifies versioning and integration of tracers.
pub trait EvmFrameInterface {
    /// Instruction pointer
    fn instruction_pointer(&self) -> usize;
    /// Resources left
    fn resources(&self) -> EvmResources;
    /// Caller address
    fn caller(&self) -> Address;
    /// Callee address
    fn address(&self) -> Address;
    /// Calldata
    fn calldata(&self) -> &[u8];
    /// Returndata is available from here if it exists
    fn return_data(&self) -> &[u8];
    /// Heap that belongs to this interpreter frame
    fn heap(&self) -> &[u8];
    /// Bytecode
    fn bytecode(&self) -> &[u8];
    /// Call value
    fn call_value(&self) -> &U256;
    /// Value of the refund counter (if enabled)
    fn refund_counter(&self) -> u32;
    /// Is EVM frame static or not.
    fn is_static(&self) -> bool;
    /// Is interpreter frame executing construction code or not.
    fn is_constructor(&self) -> bool;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EvmResources {
    pub ergs: u64,
    pub native: u64,
}

/// Result after call execution
pub enum CallResult<'a> {
    /// Call failed after preparation.
    Failed { returndata: &'a [u8] },
    /// Call succeeded.
    Successful { returndata: &'a [u8] },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(usize)]
pub enum CallModifier {
    #[default]
    NoModifier = 0,
    Constructor,
    Delegate,
    Static,
    DelegateStatic,
    ZKVMSystem,
    ZKVMSystemStatic,
    EVMCallcode,
    EVMCallcodeStatic,
}

/// Basic tracer that does nothing.
#[derive(Default)]
pub struct NopTracer;

impl EvmTracer for NopTracer {
    fn on_new_execution_frame(&mut self, _request: impl EvmRequest) {}

    fn after_execution_frame_completed(&mut self, _result: Option<(EvmResources, CallResult)>) {}

    fn on_storage_read(
        &mut self,
        _is_transient: bool,
        _address: Address,
        _key: B256,
        _value: B256,
    ) {
    }

    fn on_storage_write(
        &mut self,
        _is_transient: bool,
        _address: Address,
        _key: B256,
        _value: B256,
    ) {
    }

    fn on_bytecode_change(
        &mut self,
        _address: Address,
        _new_raw_bytecode: Option<&[u8]>,
        _new_internal_bytecode_hash: B256,
        _new_observable_bytecode_length: u32,
    ) {
    }

    fn on_event(&mut self, _address: Address, _topics: Vec<B256>, _data: &[u8]) {}

    fn begin_tx(&mut self, _calldata: &[u8]) {}

    fn finish_tx(&mut self) {}

    fn before_evm_interpreter_execution_step(
        &mut self,
        _opcode: u8,
        _frame_state: impl EvmFrameInterface,
    ) {
    }

    fn after_evm_interpreter_execution_step(
        &mut self,
        _opcode: u8,
        _frame_state: impl EvmFrameInterface,
    ) {
    }

    fn on_opcode_error(&mut self, _error: &EvmError, _frame_state: impl EvmFrameInterface) {}

    fn on_call_error(&mut self, _error: &EvmError) {}

    fn on_selfdestruct(
        &mut self,
        _beneficiary: Address,
        _token_value: U256,
        _frame_state: impl EvmFrameInterface,
    ) {
    }

    fn on_create_request(&mut self, _is_create2: bool) {}
}
