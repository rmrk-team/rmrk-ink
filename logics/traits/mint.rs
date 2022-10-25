use crate::traits::errors::RmrkError;
// use openbrush::contracts::psp34::*;
use ink_prelude::string::String;
use openbrush::{
    modifiers,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type RmrkMintableRef = dyn RmrkMintable;

#[openbrush::trait_definition]
pub trait RmrkMintable {
    /// Mint new tokens
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn mint(&mut self, to: AccountId, mint_amount: u128) -> Result<(), RmrkError>;

    // fn nft_mint_directly_to_nft(&self, parent: AccountIdOrCollectionNftTuple) -> Result<(), RmrkError>;

    /// Maximum amount of mintable tokens in this contract
    #[ink(message)]
    fn max_supply(&self) -> u128;

    /// The price to mint a single token in this contract
    #[ink(message)]
    fn price_per_mint(&self) -> Balance;

    /// Get URI from token ID
    #[ink(message)]
    fn token_uri(&self, token_id: u32) -> String;
}
