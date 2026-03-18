use crate::{Block, Bytes};
use std::{collections::HashMap, hash::Hash};

/// A generic storage interface. Caller must specify the key and value types.
pub trait Storage<K, V> {
    /// Get a value from the storage. Returns `None` if the key is not found.
    fn get(&self, key: &K) -> Option<V>;
    /// Put a value into storage.
    fn put(&mut self, key: K, value: V);
}

/// A SimpleStorage is an in-memory `Storage` implementation.
#[derive(Clone)]
pub struct SimpleStorage<K, V> {
    /// The storage of key-value pairs
    storage: HashMap<K, V>,
}

impl<K, V> Storage<K, V> for SimpleStorage<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        self.storage.get(key).cloned()
    }
    fn put(&mut self, key: K, value: V) {
        self.storage.insert(key, value);
    }
}

impl<K, V> SimpleStorage<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    pub fn new() -> Self {
        SimpleStorage {
            storage: HashMap::new(),
        }
    }
}

////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A BlockStorage is a `SimpleStorage` format used to store the
/// blocks corresponding to its hash.
#[derive(Clone)]
pub struct BlockStorage<Blk: Block> {
    /// The storage of the blocks
    storage: SimpleStorage<Bytes, Blk>,
}

impl<Blk> BlockStorage<Blk>
where
    Blk: Block + Clone,
{
    pub fn new() -> Self {
        BlockStorage {
            storage: SimpleStorage::new(),
        }
    }
}
