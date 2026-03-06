use gnet::{FullNode, Consensus, Storage, Block};
use anyhow::Error;

struct SimpleBlock;


struct SimpleConsensus;

impl Consensus for SimpleConsensus {
    type Error = Error;

    fn validate_block(&self, block: &impl Block) -> Result<(), Self::Error> {
        block.header().parent_hash();
        todo!("Validate the block");
    }
}

struct SimpleNode<C: Consensus, S: Storage> {
    consensus: C,
    storage: S,
}

impl<C: Consensus, S: Storage> FullNode for SimpleNode<C, S> {
    type Consensus = C;
    type Storage = S;

    fn new(consensus: Self::Consensus, storage: Self::Storage) -> Self {
        SimpleNode { consensus, storage }
    }

    fn start(&self) {
        todo!("Start the node");
    }
    
    fn stop(&self) {
        todo!("Stop the node");
    }
}

