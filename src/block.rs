use crate::{Bytes, Timestamp};

/// A generic block body
pub trait BlockBody {
    /// The transactions of the block
    fn transactions(&self) -> Vec<Bytes>;
}

/// A generic block header
pub trait BlockHeader {
    /// The parent hash of the block
    fn parent_hash(&self) -> Bytes;
    /// The hash of the block
    fn hash(&self) -> Bytes;
    /// The timestamp of the block
    fn timestamp(&self) -> Timestamp;
}

/// A generic block which has a `BlockBody` and a `BlockHeader`
pub trait Block<B: BlockBody, H: BlockHeader> {
    /// The body of the block
    fn body(&self) -> &B;
    /// The header of the block
    fn header(&self) -> &H;
}


////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleBody is a `BlockBody` that has a vector of transactions
#[derive(Clone)]
pub struct SimpleBody {
    transactions: Vec<Bytes>,
}

impl BlockBody for SimpleBody {
    fn transactions(&self) -> Vec<Bytes> {
        self.transactions.clone()
    }
}

impl SimpleBody {
    pub fn new(transactions: Vec<Bytes>) -> Self {
        SimpleBody { transactions }
    }
}

/// A SimpleHeader is a `BlockHeader` that has a parent hash, hash, and timestamp
#[derive(Clone)]
pub struct SimpleHeader {
    parent_hash: Bytes,
    hash: Bytes,
    timestamp: Timestamp,
}

impl BlockHeader for SimpleHeader {
    fn parent_hash(&self) -> Bytes {
        self.parent_hash
    }
    fn hash(&self) -> Bytes {
        self.hash
    }
    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl SimpleHeader {
    pub fn new(parent_hash: Bytes, hash: Bytes, timestamp: Timestamp) -> Self {
        SimpleHeader { parent_hash, hash, timestamp }
    }
}

/// A SimpleBlock is a `Block` that has a `SimpleBody` and a `SimpleHeader`. 
/// This block is used for the Gnet blockchain.
#[derive(Clone)]
pub struct SimpleBlock<B: BlockBody, H: BlockHeader> {
    body: B,
    header: H,
}

impl<B, H> Block<B, H> for SimpleBlock<B, H>
where B: BlockBody + Clone, H: BlockHeader + Clone
{
    fn body(&self) -> &B {
        &self.body
    }

    fn header(&self) -> &H {
        &self.header
    }
}

impl<B: BlockBody, H: BlockHeader> SimpleBlock<B, H> {
    pub fn new(body: B, header: H) -> Self {
        SimpleBlock { body, header }
    }
}