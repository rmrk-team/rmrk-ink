//! Trait definitions for Nesting module
use rmrk_common::{
    errors::Result,
    types::*,
};

use openbrush::{
    contracts::psp34::Id,
    traits::AccountId,
};

#[openbrush::wrapper]
pub type NestingRef = dyn Nesting;

/// Trait definitions for Nesting ink! messages
#[openbrush::trait_definition]
pub trait Nesting {
    /// Add a child NFT (from different collection) to the NFT in this collection.
    /// The status of the added child is `Pending` if caller is not owner of child NFT
    /// The status of the added child is `Accepted` if caller is is owner of child NFT
    /// The caller needs not to be the owner of the to_parent_token_id, but
    /// Caller must be owner of the child NFT,
    /// in order to perform transfer() ownership of the child nft to to_parent_token_id.
    ///
    /// # Requirements:
    /// * `child_contract_address` needs to be added by collecion owner
    /// * `to_parent_token_id` must exist.
    /// * `child_token_id` must exist.
    /// * There cannot be two identical children.
    ///
    /// # Arguments:
    /// * `to_parent_token_id`: is the tokenId of the parent NFT. The receiver of child.
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Ownership of child NFT will be transferred to this contract (cross contract call)
    /// On success emitts `RmrkEvent::ChildAdded`
    /// On success emitts `RmrkEvent::ChildAccepted` - only if caller is already owner of child NFT
    #[ink(message)]
    fn add_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<()>;

    /// Remove a child NFT (from different collection) from token_id in this collection.
    /// The status of added child is `Pending` if caller is not owner of child NFT
    /// The status of added child is `Accepted` if caller is is owner of child NFT
    ///
    /// # Requirements:
    /// * The status of the child is `Accepted`
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Ownership of child NFT will be transferred to parent NFT owner (cross contract call)
    /// On success emitts `RmrkEvent::ChildRemoved`
    #[ink(message)]
    fn remove_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<()>;

    /// Accept a child NFT (from different collection) to be owned by parent token.
    ///
    /// # Requirements:
    /// * The status of the child is `Pending`
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Child Nft is moved from pending to accepted
    /// On success emitts `RmrkEvent::ChildAccepted`
    #[ink(message)]
    fn accept_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<()>;

    /// Reject a child NFT (from different collection).
    ///
    /// # Requirements:
    /// * The status of the child is `Pending`
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Child Nft is removed from pending
    /// On success emitts `RmrkEvent::ChildRejected`
    #[ink(message)]
    fn reject_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<()>;

    /// Transfer the child NFT from one parent to another (in this collection).
    ///
    /// # Requirements:
    /// * The status of the child is `Accepted`
    ///
    /// # Arguments:
    /// * `current_parent`: current parent tokenId which holds child nft
    /// * `new_parent`: new parent tokenId which will hold child nft
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Ownership of child NFT will be transferred to this contract (cross contract call)
    /// On success emitts `RmrkEvent::ChildAdded`
    /// On success emitts `RmrkEvent::ChildAccepted` - only if caller is already owner of child NFT
    #[ink(message)]
    fn transfer_child(&mut self, from: Id, to: Id, child_nft: ChildNft) -> Result<()>;

    /// Read the number of children on the parent token.
    /// # Arguments:
    /// * `parent_token_id`: parent tokenId to check
    ///
    /// # Result:
    /// Returns the tupple of `(accepted_children, pending_children)` count
    #[ink(message)]
    fn children_balance(&self, parent_token_id: Id) -> Result<(u64, u64)>;
}

/// Trait definitions for Nesting ink events
#[openbrush::trait_definition]
pub trait NestingEvents {
    /// Emit ChildAdded event.
    fn _emit_added_child_event(&self, to: &Id, collection: &AccountId, child: &Id);

    /// Emit ChildAccepted event.
    fn _emit_child_accepted_event(
        &self,
        to: &Id,
        child_collection_address: &AccountId,
        child_token_id: &Id,
    );

    /// Emit ChildAccepted event.
    fn _emit_child_removed_event(
        &self,
        parent: &Id,
        child_collection_address: &AccountId,
        child_token_id: &Id,
    );

    /// Emit ChildRejected event.
    fn _emit_child_rejected_event(
        &self,
        parent: &Id,
        child_collection_address: &AccountId,
        child_token_id: &Id,
    );
}
