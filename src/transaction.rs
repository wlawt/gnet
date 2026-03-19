use crate::{Bytes, Timestamp, bytes, time};

/// A generic transaction format
pub trait Transaction {
    /// The signature of the transaction
    fn signature(&self) -> Bytes;
    /// The sender of the transaction
    fn sender(&self) -> Bytes;
    /// The date of the transaction
    fn date(&self) -> Timestamp;
    /// The amount of the transaction
    fn amount(&self) -> u64;
}

/// A TipTransaction is a `Transaction` that you can
/// add a priority fee to.
pub trait TipTransaction: Transaction {
    /// The priority fee of the transaction
    fn priority_fee(&self) -> u64;
}

////////////////////////////////////////////////////////////
// Mock Implementations
////////////////////////////////////////////////////////////

/// A SimpleTransaction is a `TipTransaction`
#[derive(Clone)]
pub struct SimpleTransaction {
    signature: Bytes,
    sender: Bytes,
    date: Timestamp,
    amount: u64,
    priority_fee: u64,
}

impl Transaction for SimpleTransaction {
    fn signature(&self) -> Bytes {
        self.signature
    }
    fn sender(&self) -> Bytes {
        self.sender
    }
    fn date(&self) -> Timestamp {
        self.date
    }
    fn amount(&self) -> u64 {
        self.amount
    }
}

impl TipTransaction for SimpleTransaction {
    fn priority_fee(&self) -> u64 {
        self.priority_fee
    }
}

impl SimpleTransaction {
    pub fn new(
        signature: Bytes,
        sender: Bytes,
        date: Timestamp,
        amount: u64,
        priority_fee: u64,
    ) -> Self {
        SimpleTransaction {
            signature,
            sender,
            date,
            amount,
            priority_fee,
        }
    }
}

impl Default for SimpleTransaction {
    fn default() -> Self {
        SimpleTransaction {
            signature: bytes(),
            sender: bytes(),
            date: time(),
            amount: 0,
            priority_fee: 0,
        }
    }
}
