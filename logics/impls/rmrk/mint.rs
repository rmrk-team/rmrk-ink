use crate::impls::rmrk::data;
use crate::impls::rmrk::errors::RmrkError;
pub use crate::traits::mint::{RmrkMintable, RmrkMintableRef};
use ink_prelude::string::{String, ToString};
use uniques_extension::*;

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
    /// Create new collection
    // This is just a temporary implementation since 
    // integration test will not send value in contract instantiation.
    // pallet_uniques requires some value for the collection creation
    default fn create_collection(&mut self) -> Result<(), RmrkError> {
        ink_env::debug_println!("####### creating Uniques collectionId {:?}", self.data::<data::Data>().rmrk_collection_id);
        let create_result = UniquesExt::create(
            // collection_id
            self.data::<data::Data>().rmrk_collection_id,
        );
        ink_env::debug_println!(
            "####### create_result: {:?}, last_minted_token_id={:?}",
            create_result, self.data::<data::Data>().last_minted_token_id
        );
        self.data::<data::Data>().last_minted_token_id = 0;
        Ok(())
    }
        
    /// Mint new tokens
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), RmrkError> {
        // self.data::<ownable::Data>().owner = _to;
        ink_env::debug_println!("####### mint RMRK contract for {:?}, amount:{:?}, last_minted_id {:?}", to, mint_amount, self.data::<data::Data>().last_minted_token_id);
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
            ink_env::debug_println!("####### mint in contract ({:?}, {:?})", self.data::<data::Data>().rmrk_collection_id, mint_id);
            // mint in this contract
            assert!(self
                .data::<psp34::Data>()
                ._mint_to(to, Id::U64(mint_id))
                .is_ok());
            self.data::<data::Data>().last_minted_token_id += 1;

            // mint in pallet
            let mint_result = UniquesExt::mint(
                self.data::<data::Data>().rmrk_collection_id, // collection_id
                mint_id.try_into().unwrap(),                  // item_id
                to,
            );
            ink_env::debug_println!("####### minting in pallet result = {:?}", mint_result);
        }

        Ok(())
    }

    /// Maximum amount of mintable tokens in this contract
    default fn max_supply(&self) -> u64 {
        self.data::<data::Data>().max_supply
    }

    /// The price to mint a single token in this contract
    default fn price_per_mint(&self) -> Balance {
        self.data::<data::Data>().price_per_mint
    }

    /// Get URI from token ID
    default fn token_uri(&self, token_id: u32) -> String {
        ink_env::debug_println!("####### get tokenUri for: {:?}", token_id);
        let value = self.get_attribute(
            self.data::<psp34::Data>().collection_id(),
            String::from("baseUri").into_bytes(),
        );
        let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &String::from(".json");
        ink_env::debug_println!("####### tokenUri is: [{:?}]", token_uri);
        return token_uri;
    }
}
