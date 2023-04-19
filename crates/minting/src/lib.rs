//! RMRK Minting implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod internal;
pub mod traits;
pub mod extensions {
    pub mod autoindex;
}

use extensions::autoindex::*;
use internal::Internal;

use rmrk_common::{
    errors::Result,
    roles::CONTRIBUTOR,
    utils::Utils,
};

use ink::{
    prelude::string::String as PreludeString,
    storage::Mapping,
};

use openbrush::{
    contracts::{
        access_control::{
            self,
            only_role,
        },
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

use traits::{
    Minting,
    MintingLazy,
};

pub const STORAGE_MINTING_KEY: u32 = openbrush::storage_unique_key!(MintingData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MINTING_KEY)]
pub struct MintingData {
    pub max_supply: Option<u64>,
    pub price_per_mint: Balance,
    pub nft_metadata: Mapping<Id, String>,
}

impl<T> Minting for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<access_control::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal
        + Utils,
{
    /// Mint one token to the specified account.
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn mint(&mut self, to: AccountId, token_id: Id) -> Result<()> {
        self._check_amount(1)?;
        self._mint_to(to, token_id)?;
        Ok(())
    }

    /// Assign metadata to specified token.
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn assign_metadata(&mut self, token_id: Id, metadata: String) -> Result<()> {
        self.data::<MintingData>()
            .nft_metadata
            .insert(token_id, &metadata);

        Ok(())
    }

    /// Get max supply of tokens.
    default fn max_supply(&self) -> Option<u64> {
        self.data::<MintingData>().max_supply
    }

    /// Get URI for the token Id.
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString> {
        self.ensure_exists_and_get_owner(&Id::U64(token_id))?;
        self._token_uri(token_id)
    }
}

impl<T> MintingLazy for T
where
    T: Storage<MintingData>
        + MintingAutoIndexInternal
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<extensions::autoindex::MintingAutoIndexData>
        + Storage<metadata::Data>
        + psp34::Internal
        + Utils,
{
    /// Purchase one token.
    default fn mint(&mut self) -> Result<()> {
        self._check_value(Self::env().transferred_value(), 1)?;
        MintingAutoIndexInternal::mint(self, Self::env().caller())?;
        Ok(())
    }

    /// Purchase many tokens.
    #[modifiers(non_reentrant)]
    default fn mint_many(&mut self, mint_amount: u64) -> Result<()> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        MintingAutoIndexInternal::mint_many(self, Self::env().caller(), mint_amount)?;
        Ok(())
    }

    /// Get max supply of tokens.
    default fn max_supply(&self) -> Option<u64> {
        self.data::<MintingData>().max_supply
    }

    /// Get URI for the token Id.
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString> {
        self.ensure_exists_and_get_owner(&Id::U64(token_id))?;
        self._token_uri(token_id)
    }

    /// Get token mint price.
    default fn price(&self) -> Balance {
        self.data::<MintingData>().price_per_mint
    }
}
