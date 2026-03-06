use gnet::{FullNode, SimpleFullNode, SimpleStorage, SimpleBlock, SimpleBody, SimpleHeader};
use anyhow::Error;

fn main() -> Result<(), Error> {
    let storage = SimpleStorage::<SimpleBlock<SimpleBody, SimpleHeader>>::new();
    let _node = SimpleFullNode::new(storage);

    Ok(())
}
