use crate::impls::rmrk::mint_data;
pub use crate::traits::{
    errors::RmrkError,
    mint::{RmrkMintable, RmrkMintableRef},
};

use openbrush::{
    contracts::{ownable::*, psp34::*, reentrancy_guard::*},
    modifiers,
    traits::{AccountId, Storage},
};

impl<T: Storage<ownable::Data> + Storage<mint_data::Data> + Storage<reentrancy_guard::Data>>
    RmrkMintable for T
{
    #[modifiers(non_reentrant)]
    default fn _mint_to(&mut self, _to: AccountId, _nft_id: Id) -> Result<(), RmrkError> {
        self.data::<ownable::Data>().owner = _to;
        Ok(())
    }
}
