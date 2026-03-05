use crate::{Consensus, Storage};

pub trait Node: Consensus + Storage {}