//! RMRK minting traits

use rmrk_common::errors::Result;

use ink::prelude::{
    string::String as PreludeString,
    vec::Vec,
};

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Balance,
        String,
    },
};

#[openbrush::wrapper]
pub type MintingRef = dyn Minting;

#[openbrush::wrapper]
pub type MintingLazyRef = dyn MintingLazy;

/// Trait definitions for core Minting functions
#[openbrush::trait_definition]
pub trait Minting {
    /// Mint one or more tokens.
    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> Result<Id>;

    /// Mint many to specified account.
    #[ink(message)]
    fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)>;

    /// Assign metadata to specified token.
    #[ink(message)]
    fn assign_metadata(&mut self, token_id: Id, metadata: String) -> Result<()>;

    /// Transfer token to new destination.
    #[ink(message)]
    fn transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<()>;

    /// Get max supply of tokens.
    #[ink(message)]
    fn max_supply(&self) -> Option<u64>;

    /// Get URI for the token Id.
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString>;
}

/// Trait definitions for lazy Minting functions
#[openbrush::trait_definition]
pub trait MintingLazy {
    /// Purchase one token.
    #[ink(message, payable)]
    fn mint(&mut self) -> Result<()>;

    /// Purchas many tokens.
    #[ink(message, payable)]
    fn mint_many(&mut self, mint_amount: u64) -> Result<()>;

    /// Get token mint price.
    #[ink(message)]
    fn price(&self) -> Balance;

    /// Get max supply of tokens.
    #[ink(message)]
    fn max_supply(&self) -> Option<u64>;

    /// Get URI for the token Id.
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString>;
}
