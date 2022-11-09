#![cfg_attr(not(feature = "std"), no_std)]

// #[openbrush::contract]
// pub mod shiden34 {
//     // imports from ink!
//     use ink_lang::codegen::{
//         EmitEvent,
//         Env,
//     };
//     use ink_prelude::string::{
//         String,
//         ToString,
//     };
//     use ink_storage::traits::SpreadAllocate;

//     // imports from openbrush
//     use openbrush::{
//         contracts::{
//             ownable::*,
//             psp34::{
//                 extensions::{
//                     enumerable::*,
//                     metadata::*,
//                 },
//                 Internal,
//             },
//             reentrancy_guard::*,
//         },
//         modifiers,
//         traits::Storage,
//     };

use ink_prelude::string::String;

use openbrush::{
    contracts::psp34::{extensions::enumerable::*, PSP34Error},
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type PSP34CustomRef = dyn PSP34Custom;

#[openbrush::trait_definition]
pub trait PSP34Custom {
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

    // internal functions
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;
    fn _token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}
