//! RMRK Base implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod internal;
pub mod traits;

use internal::Internal;

use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    utils::Utils,
};

use ink_prelude::string::{
    String as PreludeString,
    ToString,
};
use ink_storage::Mapping;

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

use traits::Minting;

pub const STORAGE_MINTING_KEY: u32 = openbrush::storage_unique_key!(MintingData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MINTING_KEY)]
pub struct MintingData {
    pub last_token_id: u64,
    pub max_supply: u64,
    pub price_per_mint: Balance,
    pub nft_metadata: Mapping<Id, String>,
}

impl<T> Minting for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal
        + Utils,
{
    /// Mint next available token for the caller
    default fn mint_next(&mut self) -> Result<()> {
        self._check_value(Self::env().transferred_value(), 1)?;
        let caller = Self::env().caller();
        let token_id = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(1)
            .ok_or(RmrkError::CollectionIsFull)?;
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(caller, Id::U64(token_id))?;
        self.data::<MintingData>().last_token_id += 1;

        self._emit_transfer_event(None, Some(caller), Id::U64(token_id));
        return Ok(())
    }

    /// Mint one or more tokens
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<()> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        self._check_amount(mint_amount)?;

        let next_to_mint = self.data::<MintingData>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            self.data::<psp34::Data<enumerable::Balances>>()
                ._mint_to(to, Id::U64(mint_id))?;
            self.data::<MintingData>().last_token_id += 1;
            self._emit_transfer_event(None, Some(to), Id::U64(mint_id));
        }

        Ok(())
    }

    /// Mint next available token with specific metadata
    #[modifiers(only_owner)]
    default fn mint_with_metadata(&mut self, metadata: PreludeString, to: AccountId) -> Result<()> {
        let token_id = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(1)
            .ok_or(RmrkError::CollectionIsFull)?;
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(to, Id::U64(token_id))?;
        self.data::<MintingData>()
            .nft_metadata
            .insert(Id::U64(token_id), &String::from(metadata));
        self.data::<MintingData>().last_token_id += 1;

        self._emit_transfer_event(None, Some(to), Id::U64(token_id));
        return Ok(())
    }

    /// Get max supply of tokens
    default fn max_supply(&self) -> u64 {
        self.data::<MintingData>().max_supply
    }

    /// Get token mint price
    default fn price(&self) -> Balance {
        self.data::<MintingData>().price_per_mint
    }

    /// Get URI for the token Id
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString> {
        self.ensure_exists_and_get_owner(&Id::U64(token_id))?;
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
}
