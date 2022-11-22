//! Set of functions commonly used with PSP34 contract
//!
use ink_prelude::string::String as PreludeString;

use openbrush::{
    contracts::psp34::{Id, PSP34Error},
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type Psp34CustomRef = dyn Psp34Custom;

/// Trait definitions for Psp34Custom internal functions.
pub trait Internal {
    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check if token is minted.
    fn _token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}

/// Trait definitions for Psp34Custom functions
#[openbrush::trait_definition]
pub trait Psp34Custom {
    /// Mint next available token for the caller.
    #[ink(message, payable)]
    fn mint_next(&mut self) -> Result<(), PSP34Error>;

    /// Mint one or more tokens.
    #[ink(message, payable)]
    fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Set new value for the baseUri.
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;

    /// Get URI for the token Id.
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString, PSP34Error>;

    /// Get max supply of tokens.
    #[ink(message)]
    fn max_supply(&self) -> u64;

    /// Get token mint price.
    #[ink(message)]
    fn price(&self) -> Balance;

    /// Withdraw contract's balance.
    #[ink(message)]
    fn withdraw(&mut self) -> Result<(), PSP34Error>;
}
