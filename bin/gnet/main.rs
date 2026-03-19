use anyhow::Error;
use gnet::{
    BlockStorage, FullNode, SimpleBlock, SimpleBody, SimpleFullNode, SimpleHeader,
    SimpleTransaction,
};

fn main() -> Result<(), Error> {
    let storage = BlockStorage::<
        SimpleBlock<SimpleTransaction, SimpleBody<SimpleTransaction>, SimpleHeader>,
    >::new();
    let _node = SimpleFullNode::new(storage);

    Ok(())
}
