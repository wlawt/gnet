use anyhow::Error;
use gnet::{BlockStorage, FullNode, SimpleBlock, SimpleBody, SimpleFullNode, SimpleHeader};

fn main() -> Result<(), Error> {
    let storage = BlockStorage::<SimpleBlock<SimpleBody, SimpleHeader>>::new();
    let _node = SimpleFullNode::new(storage);

    Ok(())
}
