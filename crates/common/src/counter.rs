use crate::{
    errors::{
        Error,
        Result,
        RmrkError,
    },
    types::*,
};

#[derive(Default, Debug)]
#[ink::storage_item]
pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn next(&mut self) -> Result<u32> {
        self.value = self
            .value
            .checked_add(1)
            .ok_or(Error::Rmrk(RmrkError::Overflow))?;
        Ok(self.value)
    }
}
