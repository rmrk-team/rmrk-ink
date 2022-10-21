use crate::impls::rmrk::mint_data;
pub use crate::traits::{
    errors::RmrkError,
    mint::{RmrkMintable, RmrkMintableRef},
};

use openbrush::{
    contracts::{psp34::*, reentrancy_guard::*},
    modifiers,
    traits::{AccountId, Storage},
};

impl<T> RmrkMintable for T
where
    T: Storage<mint_data::Data> + Storage<psp34::Data> + Storage<reentrancy_guard::Data>,
{
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u128) -> Result<(), RmrkError> {
        // self.data::<ownable::Data>().owner = _to;
        if mint_amount == 0 {
            return Err(RmrkError::CannotMintZeroTokens);
        }
        if self.data::<psp34::Data>().total_supply() + mint_amount
            > self.data::<mint_data::Data>().max_supply
        {
            return Err(RmrkError::CollectionFullOrLocked);
        }
        if Self::env().transferred_value() != self.data::<mint_data::Data>().price_per_mint {
            return Err(RmrkError::MintUnderpriced);
        }
        let next_to_mint = self.data::<psp34::Data>().total_supply() + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            assert!(self
                .data::<psp34::Data>()
                ._mint_to(to, Id::U32(mint_id as u32))
                .is_ok());
        }

        Ok(())
    }
}
