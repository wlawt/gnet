use crate::Block;

pub trait Consensus {
    type Error;

    fn validate_block(&self, block: &impl Block) -> Result<(), Self::Error>;
}
