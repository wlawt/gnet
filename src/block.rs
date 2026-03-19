use crate::{Bytes, Timestamp, Transaction, bytes, time};
use std::marker::PhantomData;

/// A generic block body
pub trait BlockBody {
    type T: Transaction;

    /// The transactions of the block
    fn transactions(&self) -> Vec<Self::T>;
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
pub trait Block {
    type B: BlockBody + Clone;
    type H: BlockHeader + Clone;

    /// The body of the block
    fn body(&self) -> &Self::B;
    /// The header of the block
    fn header(&self) -> &Self::H;
}

////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleBody is a `BlockBody` that has a vector of transactions
#[derive(Clone)]
pub struct SimpleBody<T> {
    transactions: Vec<T>,
}

impl<T> BlockBody for SimpleBody<T>
where
    T: Transaction + Clone,
{
    type T = T;

    fn transactions(&self) -> Vec<T> {
        self.transactions.clone()
    }
}

impl<T: Transaction + Clone> SimpleBody<T> {
    pub fn new(transactions: Vec<T>) -> Self {
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
    pub fn new(parent_hash: Bytes) -> Self {
        SimpleHeader {
            parent_hash,
            hash: bytes(),
            timestamp: time(),
        }
    }
}

/// A SimpleBlock is a `Block` that has a `SimpleBody` and a `SimpleHeader`.
/// This block is used for the Gnet blockchain.
#[derive(Clone)]
pub struct SimpleBlock<B: BlockBody, H: BlockHeader> {
    body: B,
    header: H,
}

impl<B, H> Block for SimpleBlock<B, H>
where
    B: BlockBody + Clone,
    H: BlockHeader + Clone,
{
    type B = B;
    type H = H;

    fn body(&self) -> &B {
        &self.body
    }

    fn header(&self) -> &H {
        &self.header
    }
}

impl<B, H> SimpleBlock<B, H>
where
    B: BlockBody + Clone,
    H: BlockHeader + Clone,
{
    pub fn new(body: B, header: H) -> Self {
        SimpleBlock { body, header }
    }
}
