use ink_prelude::string::String;
use openbrush::{contracts::psp34::extensions::enumerable::*, traits::AccountId};

#[openbrush::wrapper]
pub type RmrkMintableRef = dyn RmrkMintable;

#[openbrush::trait_definition]
pub trait RmrkMintable {
    #[ink(message, payable)]
    fn mint_next(&mut self) -> Result<(), PSP34Error>;
    #[ink(message, payable)]
    fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<String, PSP34Error>;
    #[ink(message)]
    fn max_supply(&self) -> u64;
    #[ink(message)]
    fn withdraw(&mut self) -> Result<(), PSP34Error>;

    /// Check if the transferred mint values is as expected
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check if token is minted
    fn _token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}
