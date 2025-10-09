pub struct Block<Header, Extrinsics> {
    // contains metadata about the block
    pub header: Header,
    // represents the transactions included in the block
    pub extrinsics: Vec<Extrinsics>,
}

// On a real blockchain, the header would also include:
// - a reference to the parent block
// - a reference to the state root
// - extrinsics root
// ..etc

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

/**
 * Represents a transaction in the blockchain.
 * an Extrinsic is literally an external message outside the blockchain
 */
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch {
    // The type used to identify the caller of the function
    type Caller;
    // The state transition function call the caller is trying to access
    type Call;

    fn dispatch(caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
