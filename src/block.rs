pub type Bytes = [u8; 32];
pub type Timestamp = u64;

pub trait BlockBody {
    fn transactions(&self) -> Vec<Bytes>;
}

pub trait BlockHeader {
    fn parent_hash(&self) -> Bytes;
    fn timestamp(&self) -> Timestamp;
}

pub trait Block {
    type Body: BlockBody;
    type Header: BlockHeader;

    fn body(&self) -> &Self::Body;
    fn header(&self) -> &Self::Header;
}

#[cfg(test)]
mod tests {
    use super::*;


    struct TestBlockBody {
        transactions: Vec<Bytes>,
    }

    impl BlockBody for TestBlockBody {
        fn transactions(&self) -> Vec<Bytes> {
            self.transactions.clone()
        }
    }
    struct TestBlockHeader {
        parent_hash: Bytes,
        timestamp: Timestamp,
    }

    impl BlockHeader for TestBlockHeader {
        fn parent_hash(&self) -> Bytes {
            self.parent_hash
        }
        fn timestamp(&self) -> Timestamp {
            self.timestamp
        }
    }

    struct TestBlock {
        body: TestBlockBody,
        header: TestBlockHeader,
    }

    impl Block for TestBlock {
        type Body = TestBlockBody;
        type Header = TestBlockHeader;

        fn body(&self) -> &Self::Body {
            &self.body
        }

        fn header(&self) -> &Self::Header {
            &self.header
        }
    }

    #[test]
    fn test_block_header() {
        let block = TestBlock {
            body: TestBlockBody {
                transactions: Vec::new(),
            },
            header: TestBlockHeader {
                parent_hash: Bytes::default(),
                timestamp: Timestamp::default(),
            },
        };
        assert_eq!(block.body.transactions(), Vec::<Bytes>::new());
        assert_eq!(block.header.parent_hash(), Bytes::default());
        assert_eq!(block.header.timestamp(), Timestamp::default());
    }
}