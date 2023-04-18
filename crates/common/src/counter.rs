use crate::errors::{
    Error,
    Result,
    RmrkError,
};

use openbrush::contracts::psp34::extensions::enumerable::*;

#[derive(Debug, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Counter<T>
where
    T: Default + core::fmt::Debug + scale::Encode + scale::Decode,
{
    value: T,
}

pub trait Increment<T = Self> {
    fn inc(&mut self) -> Result<T>;
}

impl Increment for u32 {
    fn inc(&mut self) -> Result<u32> {
        *self = self
            .checked_add(1)
            .ok_or(Error::Rmrk(RmrkError::Overflow))?;
        Ok(*self)
    }
}

impl Increment for Id {
    fn inc(&mut self) -> Result<Id> {
        *self = match self {
            Id::U8(id) => id.checked_add(1).map(Id::U8),
            Id::U16(id) => id.checked_add(1).map(Id::U16),
            Id::U32(id) => id.checked_add(1).map(Id::U32),
            Id::U64(id) => id.checked_add(1).map(Id::U64),
            Id::U128(id) => id.checked_add(1).map(Id::U128),
            Id::Bytes(_) => panic!("bytes id increment unsupported"),
        }
        .ok_or(Error::Rmrk(RmrkError::Overflow))?;
        Ok(self.clone())
    }
}

impl Counter<u32> {
    pub fn current(&self) -> u32 {
        self.value
    }

    pub fn next(&mut self) -> Result<u32> {
        self.value.inc()
    }
}

impl Counter<Id> {
    pub fn current(&self) -> Id {
        self.value.clone()
    }

    pub fn next(&mut self) -> Result<Id> {
        self.value.inc()
    }
}

impl Default for Counter<u32> {
    fn default() -> Self {
        Counter { value: 0 }
    }
}

impl Default for Counter<Id> {
    fn default() -> Self {
        Counter { value: Id::U64(0) }
    }
}
