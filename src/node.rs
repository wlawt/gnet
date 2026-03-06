use crate::{Consensus, Storage};

pub trait FullNode {
    type Consensus: Consensus;
    type Storage: Storage;

    fn new(consensus: Self::Consensus, storage: Self::Storage) -> Self;
    fn start(&self);
    fn stop(&self);
}
