//! RMRK minting traits

use rmrk_common::errors::Result;

use ink_prelude::string::String as PreludeString;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type MintingRef = dyn Minting;

/// Trait definitions for Minting functions
#[openbrush::trait_definition]
pub trait Minting {
    /// Mint next available token for the caller.
    #[ink(message, payable)]
    fn mint_next(&mut self) -> Result<()>;

    /// Mint one or more tokens.
    #[ink(message, payable)]
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<()>;

    /// Mint next available token with specific metadata
    #[ink(message)]
    fn mint_with_metadata(&mut self, metadata: PreludeString, to: AccountId) -> Result<()>;

    /// Get max supply of tokens.
    #[ink(message)]
    fn max_supply(&self) -> u64;

    /// Get token mint price.
    #[ink(message)]
    fn price(&self) -> Balance;

    /// Get URI for the token Id.
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString>;
}
