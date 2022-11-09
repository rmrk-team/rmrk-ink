use crate::impls::rmrk::data;
use crate::impls::rmrk::errors::RmrkError;
pub use crate::traits::mint::{RmrkMintable, RmrkMintableRef};
use ink_prelude::string::{String, ToString};

use openbrush::{
    contracts::{psp34::*, reentrancy_guard::*},
    modifiers,
    traits::{AccountId, Balance, Storage},
};

impl<T> RmrkMintable for T
where
    T: Storage<data::Data>
        + Storage<psp34::Data>
        + Storage<reentrancy_guard::Data>
        + psp34::extensions::metadata::PSP34Metadata,
{
    /// Mint new tokens
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), RmrkError> {
        // self.data::<ownable::Data>().owner = _to;
        ink_env::debug_println!("####### mint RMRK contract amount:{:?}", mint_amount);
        if mint_amount == 0 {
            return Err(RmrkError::CannotMintZeroTokens);
        }
        if self.data::<data::Data>().last_minted_token_id + mint_amount
            > self.data::<data::Data>().max_supply
        {
            ink_env::debug_println!("####### error CollectionFullOrLocked");
            return Err(RmrkError::CollectionFullOrLocked);
        }
        if Self::env().transferred_value()
            != mint_amount as u128 * self.data::<data::Data>().price_per_mint
        {
            ink_env::debug_println!("####### error MintUnderpriced");
            return Err(RmrkError::MintUnderpriced);
        }

        let next_to_mint = self.data::<data::Data>().last_minted_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            ink_env::debug_println!("####### mint id:{:?}", mint_id);
            // mint in this contract
            let mint_result = self
                .data::<psp34::Data>()
                ._mint_to(to, Id::U64(mint_id));
            self.data::<data::Data>().last_minted_token_id += 1;

            ink_env::debug_println!("####### minting mint_result: {:?}", mint_result);
        }

        Ok(())
    }
}
