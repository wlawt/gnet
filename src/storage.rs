use crate::{Block, Bytes, BlockBody, BlockHeader};
use anyhow::Error;
use std::collections::HashMap;

/// A generic storage interface
pub trait Storage<B: BlockBody, H: BlockHeader, Blk: Block<B, H>> {
    type Error;

    fn put_block(&mut self, block: &Blk) -> Result<(), Self::Error>;
    fn get_block(&self, hash: &Bytes) -> Result<Option<Blk>, Self::Error>;
}


////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleStorage is a `Storage` that stores blocks in a HashMap
struct SimpleStorage<Blk> {
    blocks: HashMap<Bytes, Blk>,
}

impl<B, H, Blk> Storage<B, H, Blk> for SimpleStorage<Blk>
where 
    B: BlockBody, 
    H: BlockHeader,
    Blk: Block<B, H> + Clone,
{
    type Error = Error;

    fn put_block(&mut self, block: &Blk) -> Result<(), Self::Error> {
        self.blocks.insert(block.header().hash(), block.clone());
        Ok(())
    }

    fn get_block(&self, hash: &Bytes) -> Result<Option<Blk>, Self::Error> {
        Ok(self.blocks.get(hash).cloned())
    }
}