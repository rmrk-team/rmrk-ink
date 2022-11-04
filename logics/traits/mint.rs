use crate::impls::rmrk::errors::RmrkError;
use ink_prelude::string::String;
use openbrush::{
    modifiers,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type RmrkMintableRef = dyn RmrkMintable;

#[openbrush::trait_definition]
pub trait RmrkMintable {
    /// Create new collection
    #[ink(message, payable)]
    fn create_collection(&mut self) -> Result<(), RmrkError>;

    /// Mint new tokens
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), RmrkError>;

    // #[ink(message)]
    // fn nft_mint_directly_to_nft(&self, parent: AccountIdOrCollectionNftTuple) -> Result<(), RmrkError>;

    /// Maximum amount of mintable tokens in this contract
    #[ink(message)]
    fn max_supply(&self) -> u64;

    /// The price to mint a single token in this contract
    #[ink(message)]
    fn price_per_mint(&self) -> Balance;

    /// Get URI from token ID
    #[ink(message)]
    fn token_uri(&self, token_id: u32) -> String;
}
