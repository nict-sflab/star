use alloc::vec::Vec;

use crate::{
    Result,
    codec::{append_blob, take_blob},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence<'a> {
    pub quote: &'a [u8],
    pub collateral: &'a [u8],
}

impl<'a> Evidence<'a> {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        append_blob(&mut out, self.quote);
        append_blob(&mut out, self.collateral);
        out
    }

    pub fn decode(payload: &'a [u8]) -> Result<Self> {
        let mut remaining = payload;
        let quote = take_blob(&mut remaining)?;
        let collateral = take_blob(&mut remaining)?;
        if !remaining.is_empty() {
            return Err("trailing raw quote evidence bytes".into());
        }
        Ok(Self { quote, collateral })
    }
}
