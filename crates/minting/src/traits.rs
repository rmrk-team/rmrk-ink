//! RMRK minting traits

use rmrk_common::errors::Result;

use ink::prelude::string::String as PreludeString;
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

#[openbrush::wrapper]
pub type MintingAutoIndexRef = dyn MintingAutoIndex;

/// Trait definitions for core Minting functions
#[openbrush::trait_definition]
pub trait Minting {
    /// Mint one or more tokens.
    #[ink(message)]
    fn mint(&mut self, to: AccountId, token_id: Id) -> Result<()>;

    /// Assign metadata to specified token.
    #[ink(message)]
    fn assign_metadata(&mut self, token_id: Id, metadata: String) -> Result<()>;

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

/// Trait definitions for MintingAutoIndex functions
#[openbrush::trait_definition]
pub trait MintingAutoIndex {
    /// Mint one token to the specified account, with auto-generated Id
    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> Result<Id>;

    /// Mint one or more tokens to the specified account, with auto-generated Ids
    #[ink(message)]
    fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)>;
}

