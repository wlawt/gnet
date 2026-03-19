use crate::{Block, BlockHeader, SimpleBlock, SimpleBody, SimpleHeader, Transaction};
use anyhow::Result;

pub trait Consensus<B, T> {
    fn produce_block(&mut self, parent_block: &B, txs: Vec<T>) -> Result<B>;
}

////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleConsensus is a `Consensus` that validates and produces blocks.
#[derive(Clone)]
pub struct SimpleConsensus<B> {
    /// Chain of blocks
    chain: Vec<B>,
}

impl<T> Consensus<SimpleBlock<SimpleBody<T>, SimpleHeader>, T>
    for SimpleConsensus<SimpleBlock<SimpleBody<T>, SimpleHeader>>
where
    T: Transaction + Clone,
{
    fn produce_block(
        &mut self,
        parent_block: &SimpleBlock<SimpleBody<T>, SimpleHeader>,
        txs: Vec<T>,
    ) -> Result<SimpleBlock<SimpleBody<T>, SimpleHeader>> {
        let parent_hash = parent_block.header().parent_hash();

        let header = SimpleHeader::new(parent_hash);
        let body = SimpleBody::new(txs);

        let block = SimpleBlock::new(body, header);
        self.chain.push(block.clone());

        Ok(block)
    }
}
