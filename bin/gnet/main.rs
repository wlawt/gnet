use anyhow::Error;
use gnet::{
    BlockStorage, FullNode, SimpleBlock, SimpleBody, SimpleConsensus, SimpleFullNode, SimpleHeader,
    SimpleTransaction,
};
use tracing::info;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    info!("Starting Gnet node");
    let consensus =
        SimpleConsensus::<SimpleBlock<SimpleBody<SimpleTransaction>, SimpleHeader>>::new();
    let mut node = SimpleFullNode::new(consensus);
    node.run()?;

    Ok(())
}
