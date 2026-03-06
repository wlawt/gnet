use crate::{Storage, Block, SimpleStorage};

/// A full node is a node that can validate blocks and store them in a storage.
pub trait FullNode<Blk: Block> {
    type Storage: Storage<Blk>;

    /// Create a new full node
    fn new(storage: Self::Storage) -> Self;
}


////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleFullNode is a `FullNode` that can validate blocks and store them in a storage.
#[derive(Clone)]
pub struct SimpleFullNode<Blk: Block> {
    /// The storage of the full node
    blocks: SimpleStorage<Blk>,
}

impl<Blk: Block + Clone> FullNode<Blk> for SimpleFullNode<Blk> {
    type Storage = SimpleStorage<Blk>;

    fn new(storage: Self::Storage) -> Self {
        SimpleFullNode { blocks: storage }
    }
}
