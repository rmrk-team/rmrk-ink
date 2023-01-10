//! Set of functions commonly used with PSP34 contract
use ink_prelude::string::String as PreludeString;

use openbrush::{
    contracts::psp34::{
        Id,
        PSP34Error,
    },
    traits::{
        AccountId,
        Balance,
    },
};

/// Trait definitions for Utils internal functions.
pub trait Internal {
    /// Check if token is minted.
    fn _token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}

/// Trait definitions for Utils functions
#[openbrush::trait_definition]
pub trait Utils {
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

    /// Ensure that token exists
    fn ensure_exists(&self, id: &Id) -> Result<AccountId, PSP34Error>;

    /// Ensure that the caller is the token owner
    fn ensure_token_owner(&self, token_owner: AccountId) -> Result<(), PSP34Error>;
}
