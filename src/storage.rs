use crate::{Block, Bytes, BlockHeader};
use anyhow::Error;
use std::collections::HashMap;

/// A generic storage interface
pub trait Storage<Blk> {
    type Error;

    fn put_block(&mut self, block: &Blk) -> Result<(), Self::Error>;
    fn get_block(&self, hash: &Bytes) -> Result<Option<Blk>, Self::Error>;
}


////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleStorage is a `Storage` that stores blocks in a HashMap
#[derive(Clone)]
pub struct SimpleStorage<Blk> {
    /// The chain of blocks
    blocks: HashMap<Bytes, Blk>,
}

impl<Blk> Storage<Blk> for SimpleStorage<Blk>
where
    Blk: Block + Clone,
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

impl<Blk> SimpleStorage<Blk>
where 
    Blk: Block + Clone,
{
    pub fn new() -> Self {
        SimpleStorage { blocks: HashMap::new() }
    }
}
