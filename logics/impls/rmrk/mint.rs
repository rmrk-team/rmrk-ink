use crate::impls::rmrk::data;
pub use crate::traits::{
    errors::RmrkError,
    mint::{RMRKMintable, RMRKMintableRef},
};

use openbrush::{
    contracts::{ownable::*, reentrancy_guard::*},
    modifiers,
    traits::{AccountId, Storage},
};

impl<T: Storage<ownable::Data> + Storage<data::Data> + Storage<reentrancy_guard::Data>> RMRKMintable
    for T
{
    #[modifiers(non_reentrant)]
    default fn mint_multiple(&mut self, _to: AccountId, _amount: u16) -> Result<(), RmrkError> {
        self.data::<data::Data>().owner = _to;
        Ok(())
    }
}
