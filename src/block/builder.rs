use super::{Block, SIZEOF_U16};
use bytes::BufMut;

/// Build a block
pub struct BlockBuilder {
    /// All key value pairs in the block
    data: Vec<u8>,
    /// Offsets of each key-value entries
    offsets: Vec<u16>,
    /// the expected block size
    block_size: usize,
}

impl BlockBuilder {
    /// Create new block builder
    pub fn new(block_size: usize) -> Self {
        BlockBuilder {
            data: Vec::new(),
            offsets: Vec::new(),
            block_size,
        }
    }

    fn estimated_size(&self) -> usize {
        self.offsets.len() * SIZEOF_U16 + self.data.len() + SIZEOF_U16
    }

    /// Add key-value pair to the block. Returns false when the block is full
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        assert!(!key.is_empty(), "key must not be empty");

        // the overhead here is key_len + val_len + offset, each is of type u16
        if self.estimated_size() + key.len() + value.len() + SIZEOF_U16 * 3 > self.block_size
            && !self.is_empty()
        {
            return false;
        }

        // update offset first, to maintain the correct offsets
        self.offsets.push(self.data.len() as u16);
        self.data.put_u16(key.len() as u16);
        self.data.put(key);
        self.data.put_u16(value.len() as u16);
        self.data.put(value);

        true
    }

    /// Check if there is no key-value pair in the block
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    pub fn build(self) -> Block {
        if self.is_empty() {
            panic!("block should not be empty!");
        }

        Block {
            data: self.data,
            offsets: self.offsets,
        }
    }
}
