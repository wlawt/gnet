use crate::{Block, Bytes};

pub trait Storage {
    type Error;

    fn put_block(&self, block: &impl Block) -> Result<(), Self::Error>;
    fn get_block(&self, hash: &Bytes) -> Result<Option<impl Block>, Self::Error>;
}