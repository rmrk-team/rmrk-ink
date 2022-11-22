use crate::impls::rmrk::types::*;
use openbrush::{contracts::psp34::Id, contracts::psp34::PSP34Error, traits::AccountId};

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
    #[ink(message)]
    fn children_balance(&self) -> Result<(u64, u64), PSP34Error>;
}

#[openbrush::trait_definition]
pub trait NestingEvents {
    /// Emit AddedChild event
    fn _emit_added_child_event(&self, to: Id, collection: AccountId, child: Id);
    /// Emit ChildAccepted event
    fn _emit_child_accepted_event(
        &self,
        to: Id,
        child_collection_address: AccountId,
        child_token_id: Id,
    );
    /// Emit ChildAccepted event
    fn _emit_child_removed_event(
        &self,
        parent: Id,
        child_collection_address: AccountId,
        child_token_id: Id,
    );
    /// Emit ChildRejected event
    fn _emit_child_rejected_event(
        &self,
        parent: Id,
        child_collection_address: AccountId,
        child_token_id: Id,
    );
}

pub trait Internal {
    /// Check if child is already accepted
    fn already_accepted(&self, parent_token_id: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;

    /// Check if child is already pending
    fn already_pending(&self, parent_token_id: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;

    /// Add the child to the list of accepted children
    fn add_to_accepted(&mut self, parent_token_id: Id, child_nft: ChildNft);

    /// Remove the child to the list of accepted children
    fn remove_accepted(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error>;

    /// Add the child to the list of pending children
    fn add_to_pending(&mut self, parent_token_id: Id, child_nft: ChildNft);

    /// Remove the child to the list of pending children
    fn remove_from_pending(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error>;

    /// Check if token is minted. Return the owner
    fn ensure_exists(&self, id: Id) -> Result<AccountId, PSP34Error>;

    /// Check if caller is the owner of this parent token
    fn is_caller_parent_owner(
        &self,
        caller: AccountId,
        parent_token_id: Id,
    ) -> Result<(), PSP34Error>;

    /// Cross contract call to transfer child nft ownership
    fn transfer_child_ownership(
        &self,
        to: AccountId,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error>;
}
