//! Set of functions commonly used with PSP34 contract

use ink_prelude::string::{
    String as PreludeString,
    ToString,
};

use crate::impls::rmrk::{
    errors::RmrkError,
    types::MintingData,
};
pub use crate::traits::utils::{
    Internal,
    Utils,
};
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
    },
};

impl<T> Utils for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    /// Set new value for the baseUri
    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        let id = self
            .data::<psp34::Data<enumerable::Balances>>()
            .collection_id();
        self.data::<metadata::Data>()
            ._set_attribute(id, String::from("baseUri"), uri.into_bytes());
        Ok(())
    }

    /// Get URI for the token Id
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString, PSP34Error> {
        self.ensure_exists(&Id::U64(token_id))?;
        let uri: PreludeString;
        match self
            .data::<MintingData>()
            .nft_metadata
            .get(Id::U64(token_id))
        {
            Some(token_uri) => {
                uri = PreludeString::from_utf8(token_uri).unwrap();
            }
            None => {
                let value = self.get_attribute(
                    self.data::<psp34::Data<enumerable::Balances>>()
                        .collection_id(),
                    String::from("baseUri"),
                );
                let token_uri = PreludeString::from_utf8(value.unwrap()).unwrap();
                uri = token_uri + &token_id.to_string() + &PreludeString::from(".json");
            }
        }
        Ok(uri)
    }

    /// Get max supply of tokens
    default fn max_supply(&self) -> u64 {
        self.data::<MintingData>().max_supply
    }

    /// Get token mint price
    default fn price(&self) -> Balance {
        self.data::<MintingData>().price_per_mint
    }

    /// Withdraw contract's balance
    #[modifiers(only_owner)]
    default fn withdraw(&mut self) -> Result<(), PSP34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        Self::env()
            .transfer(self.data::<ownable::Data>().owner(), current_balance)
            .map_err(|_| PSP34Error::Custom(String::from(RmrkError::WithdrawalFailed.as_str())))?;
        Ok(())
    }
    /// Check if token is minted. Return the owner
    default fn ensure_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        let token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Ensure that the caller is the token owner
    default fn ensure_token_owner(&self, token_owner: AccountId) -> Result<(), PSP34Error> {
        let caller = Self::env().caller();
        if caller != token_owner {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::NotTokenOwner.as_str(),
            )))
        }
        Ok(())
    }
}
