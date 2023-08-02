//! RMRK Minting implementation
#![cfg_attr(not(feature = "std"), no_std)]

mod internal;

use ink::primitives::AccountId;
use internal::Internal;

use rmrk_traits::psp34::{
    PSP34Error,
    PSP34,
};

use rmrk_storage::RmrkStorageSelector;

pub trait Minting: RmrkStorageSelector {
    fn mint(&mut self, to: AccountId) -> Result<(), PSP34Error>;
}

impl<T> Minting for T
where
    T: RmrkStorageSelector + Internal,
{
    fn mint(&mut self, to: AccountId) -> Result<(), PSP34Error> {
        self._check_value(0, 0);
        self.storage().psp34.mint(to, 0)?;
        Ok(())
    }
}
