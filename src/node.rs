use crate::{Storage, Block, BlockStorage, SimpleStorage, Bytes};

/// A full node is a node that can validate blocks and store them in a storage.
pub trait FullNode<S> {
    /// Create a new full node
    fn new(storage: S) -> Self;
}


////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleFullNode is a `FullNode` that can validate blocks and store them in a storage.
#[derive(Clone)]
pub struct SimpleFullNode<Blk: Block> {
    /// The storage of the full node
    blocks: BlockStorage<Blk>,
}

impl<Blk> FullNode<BlockStorage<Blk>> for SimpleFullNode<Blk>
where Blk: Block + Clone, 
{
    fn new(storage: BlockStorage<Blk>) -> Self {
        SimpleFullNode { blocks: storage }
    }
}
