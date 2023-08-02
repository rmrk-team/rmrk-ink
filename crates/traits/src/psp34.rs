#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;

// Note:
//
// For ease-of-use, simplicity and adoption keep to native rust types. 
// The main issue is u256 compat with EVM, but do we really want to
// complicate the core ink impl with this? IMO core PSP impls should be
// as simple as possible to allow minimal friction - "extra" functionality 
// should come later.
pub type Balance = u128;
pub type Id = u128;

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
)]
pub enum PSP34Error {
    Example,
}

// Note:
//
// This should be defined and agreed spec for PSP34 and should
// live in a PSP repo. The PSP34 crate should also contain
// an implementation, and perhaps a registry for already deployed PSP34
// instances on various chains (so users can reuse stored code).
#[ink::trait_definition]
pub trait PSP34 {
    #[ink(message)]
    fn collection_id(&self) -> Id;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
