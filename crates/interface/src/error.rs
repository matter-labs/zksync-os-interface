use alloy::primitives::{Address, U256};

// Taken from revm, contains changes
///
/// Transaction validation error.
///
#[derive(Debug, Clone)]
pub enum InvalidTransaction {
    /// Failed to decode.
    InvalidEncoding,
    /// Fields set incorrectly in accordance to its type.
    InvalidStructure,
    /// When using the EIP-1559 fee model introduced in the London upgrade, transactions specify two primary fee fields:
    /// - `gas_max_fee`: The maximum total fee a user is willing to pay, inclusive of both base fee and priority fee.
    /// - `gas_priority_fee`: The extra amount a user is willing to give directly to the miner, often referred to as the "tip".
    ///
    /// Provided `gas_priority_fee` exceeds the total `gas_max_fee`.
    PriorityFeeGreaterThanMaxFee,
    /// `basefee` is greater than provided `gas_max_fee`.
    BaseFeeGreaterThanMaxFee,
    /// EIP-1559: `gas_price` is less than `basefee`.
    GasPriceLessThanBasefee,
    /// `gas_limit` in the tx is bigger than `block_gas_limit`.
    CallerGasLimitMoreThanBlock,
    /// Initial gas for a Call is bigger than `gas_limit`.
    ///
    /// Initial gas for a Call contains:
    /// - initial stipend gas
    /// - gas for access list and input data
    CallGasCostMoreThanGasLimit,
    /// EIP-3607 Reject transactions from senders with deployed code
    RejectCallerWithCode,
    /// Transaction account does not have enough amount of ether to cover transferred value and gas_limit*gas_price.
    LackOfFundForMaxFee {
        fee: U256,
        balance: U256,
    },
    /// Overflow payment in transaction.
    OverflowPaymentInTransaction,
    /// Nonce overflows in transaction.
    NonceOverflowInTransaction,
    NonceTooHigh {
        tx: u64,
        state: u64,
    },
    NonceTooLow {
        tx: u64,
        state: u64,
    },
    MalleableSignature,
    IncorrectFrom {
        tx: Address,
        recovered: Address,
    },
    /// EIP-3860: Limit and meter initcode
    CreateInitCodeSizeLimit,
    /// Transaction chain id does not match the config chain id.
    InvalidChainId,
    /// Access list is not supported for blocks before the Berlin hardfork.
    AccessListNotSupported,
    /// Unacceptable gas per pubdata price.
    GasPerPubdataTooHigh,
    /// Block gas limit is too high.
    BlockGasLimitTooHigh,
    /// Protocol upgrade tx should be first in the block.
    UpgradeTxNotFirst,

    /// Call during AA validation reverted
    Revert {
        method: AAMethod,
        output: Option<&'static [u8]>,
    },
    /// Bootloader received insufficient fees
    ReceivedInsufficientFees {
        received: U256,
        required: U256,
    },
    /// Invalid magic returned by validation
    InvalidMagic,
    /// Validation returndata is of invalid length
    InvalidReturndataLength,
    /// Ran out of gas during validation
    OutOfGasDuringValidation,
    /// Ran out of native resources during validation
    OutOfNativeResourcesDuringValidation,
    /// Transaction nonce already used
    NonceUsedAlready,
    /// Nonce not increased after validation
    NonceNotIncreased,
    /// Return data from paymaster is too short
    PaymasterReturnDataTooShort,
    /// Invalid magic in paymaster validation
    PaymasterInvalidMagic,
    /// Paymaster returned invalid context
    PaymasterContextInvalid,
    /// Paymaster context offset is greater than returndata length
    PaymasterContextOffsetTooLong,
    /// Transaction makes the block reach the gas limit
    BlockGasLimitReached,
    /// Transaction makes the block reach the native resource limit
    BlockNativeLimitReached,
    /// Transaction makes the block reach the pubdata limit
    BlockPubdataLimitReached,
    /// Transaction makes the block reach the l2->l1 logs limit
    BlockL2ToL1LogsLimitReached,
}

/// Methods called during AA validation
#[derive(Debug, Clone)]
pub enum AAMethod {
    /// The account's validation method itself
    AccountValidate,
    /// The account's pay for transaction method
    AccountPayForTransaction,
    /// The account's pre paymaster method
    AccountPrePaymaster,
    /// Paymaster payment
    PaymasterValidateAndPay,
}

// We don't need anything more than Debug here -- the error should be passed to
// the sequencer, converted to an appropriate public error through zksync-error
// framework and then passed to the clients.
impl core::fmt::Display for InvalidTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
