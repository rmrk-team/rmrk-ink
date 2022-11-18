use crate::impls::rmrk::types::*;
use openbrush::{contracts::psp34::Id, contracts::psp34::PSP34Error};

#[openbrush::wrapper]
pub type NestingRef = dyn Nesting;

#[openbrush::trait_definition]
pub trait Nesting {
    #[ink(message)]
    fn add_child(&mut self, parent_token_is: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn remove_child(&mut self, parent_token_is: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn accept_child(&mut self, parent_token_is: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn reject_child(&mut self, parent_token_is: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn transfer_child(&mut self, from: Id, to: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
}
