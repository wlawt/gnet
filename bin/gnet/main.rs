use gnet::{FullNode, SimpleFullNode, SimpleBlock, SimpleBody, SimpleHeader, BlockStorage};
use anyhow::Error;

fn main() -> Result<(), Error> {
    let storage = BlockStorage::<SimpleBlock<SimpleBody, SimpleHeader>>::new();
    let _node = SimpleFullNode::new(storage);

    Ok(())
}
