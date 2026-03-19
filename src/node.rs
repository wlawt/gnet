use crate::{
    Block, BlockHeader, Bytes, Consensus, SimpleBlock, SimpleBody, SimpleConsensus, SimpleHeader,
    SimpleTransaction, Storage, Transaction, bytes,
};
use anyhow::Result;
use tracing::info;

/// A full node is a node that can validate blocks and store them in a storage.
pub trait FullNode<C> {
    /// Create a new full node
    fn new(consensus: C) -> Self;
}

////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleFullNode is a `FullNode` that can validate blocks and store them in a storage.
#[derive(Clone)]
pub struct SimpleFullNode<B: Block> {
    /// The storage of the full node
    consensus: SimpleConsensus<B>,
}

impl<B> FullNode<SimpleConsensus<B>> for SimpleFullNode<B>
where
    B: Block + Clone,
{
    fn new(consensus: SimpleConsensus<B>) -> Self {
        SimpleFullNode { consensus }
    }
}

impl SimpleFullNode<SimpleBlock<SimpleBody<SimpleTransaction>, SimpleHeader>> {
    pub fn run(&mut self) -> Result<()> {
        let genesis_hash = bytes();
        let genesis_header = SimpleHeader::new(genesis_hash);
        let genesis_body = SimpleBody::new(vec![]);
        let genesis_block = SimpleBlock::new(genesis_body, genesis_header);
        info!(
            "Produced genesis block: {:?}",
            genesis_block.header().hash()
        );

        self.consensus.produce_block(&genesis_block, vec![])?;

        let mut parent = genesis_block;
        let mut txs = vec![SimpleTransaction::default()];
        loop {
            let next = self.consensus.produce_block(&parent, txs)?;
            info!("Produced block: {:?}", next.header().hash());
            parent = next;
            txs = vec![SimpleTransaction::default()];
        }
    }
}
