//! Starknet block definition.

use super::header::Header;

/// Starknet block definition.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    scale_codec::Encode,
    scale_codec::Decode,
    scale_info::TypeInfo,
    Default,
    scale_codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// The block header.
    header: Header,
}

impl Block {
    /// Creates a new block.
    pub fn new(header: Header) -> Self {
        Self { header }
    }

    /// Return a reference to the block header
    pub fn header(&self) -> &Header {
        &self.header
    }
}