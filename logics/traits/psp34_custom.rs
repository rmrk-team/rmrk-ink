#![cfg_attr(not(feature = "std"), no_std)]

use ink_prelude::string::String;

use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type Psp34CustomRef = dyn Psp34Custom;

#[openbrush::trait_definition]
pub trait Psp34Custom {
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
    fn price(&self) -> Balance;
    #[ink(message)]
    fn withdraw(&mut self) -> Result<(), PSP34Error>;
}
