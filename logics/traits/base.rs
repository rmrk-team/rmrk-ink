//! RMRK minting traits

use openbrush::{
    contracts::psp34::PSP34Error,
    traits::AccountId,
};

#[openbrush::wrapper]
pub type MintingRef = dyn Minting;

/// Trait definitions for Minting internal functions.
pub trait Internal {
    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;
}

/// Trait definitions for Minting functions
#[openbrush::trait_definition]
pub trait Minting {
    /// Mint next available token for the caller.
    #[ink(message, payable)]
    fn mint_next(&mut self) -> Result<(), PSP34Error>;

    /// Mint one or more tokens.
    #[ink(message, payable)]
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error>;
}
