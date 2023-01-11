//! This module enables nesting of RMRK or any other NFT which inherits PSP34.

use crate::impls::rmrk::{
    errors::RmrkError,
    types::*,
};
pub use crate::traits::{
    nesting::{
        Internal,
        Nesting,
        NestingEvents,
    },
    utils::Utils,
};
use ink_env::CallFlags;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Implement internal helper trait for Nesting
impl<T> Internal for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if child is already accepted
    default fn accepted(
        &self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self
            .data::<NestingData>()
            .accepted_children
            .get(parent_token_id)
        {
            if children.contains(child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedChild.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if child is already pending
    default fn pending(
        &self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self
            .data::<NestingData>()
            .pending_children
            .get(parent_token_id)
        {
            if children.contains(child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingChild.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Add the child to the list of accepted children
    default fn add_to_accepted(&mut self, parent_token_id: Id, child_nft: ChildNft) {
        let mut child_nfts = self
            .data::<NestingData>()
            .accepted_children
            .get(&parent_token_id)
            .unwrap_or(Vec::new());
        if !child_nfts.contains(&child_nft) {
            child_nfts.push(child_nft.clone());
            self.data::<NestingData>()
                .accepted_children
                .insert(&parent_token_id, &child_nfts);
            self._emit_child_accepted_event(&parent_token_id, &child_nft.0, &child_nft.1);
        }
    }

    /// Remove the child to the list of accepted children
    default fn remove_accepted(
        &mut self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        let mut child_nfts = self
            .data::<NestingData>()
            .accepted_children
            .get(&parent_token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidParentId.as_str(),
            )))?;

        let index = child_nfts
            .iter()
            .position(|x| x == child_nft)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::ChildNotFound.as_str(),
            )))?;
        child_nfts.remove(index);

        self.data::<NestingData>()
            .accepted_children
            .insert(&parent_token_id, &child_nfts);

        self._emit_child_removed_event(&parent_token_id, &child_nft.0, &child_nft.1);
        Ok(())
    }

    /// Add the child to the list of pending children
    default fn add_to_pending(&mut self, parent_token_id: Id, child_nft: ChildNft) {
        let mut child_nfts = self
            .data::<NestingData>()
            .pending_children
            .get(&parent_token_id)
            .unwrap_or(Vec::new());
        if !child_nfts.contains(&child_nft) {
            child_nfts.push(child_nft);
            self.data::<NestingData>()
                .pending_children
                .insert(&parent_token_id, &child_nfts);
        }
    }

    /// Remove the child to the list of pending children
    default fn remove_from_pending(
        &mut self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        let mut child_nfts = self
            .data::<NestingData>()
            .pending_children
            .get(&parent_token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidParentId.as_str(),
            )))?;

        let index = child_nfts
            .iter()
            .position(|x| x == child_nft)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::ChildNotFound.as_str(),
            )))?;
        child_nfts.remove(index);

        self.data::<NestingData>()
            .pending_children
            .insert(&parent_token_id, &child_nfts);

        Ok(())
    }

    /// Check if caller is the owner of this parent token
    default fn is_caller_parent_owner(
        &self,
        caller: AccountId,
        parent_token_id: &Id,
    ) -> Result<(), PSP34Error> {
        if let Some(token_owner) = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(parent_token_id.clone())
        {
            if token_owner != caller {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::NotTokenOwner.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Cross contract call to transfer child nft ownership
    default fn transfer_child_ownership(
        &self,
        to: AccountId,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        // TODO check child collection is approved by this (parent) collection
        // let collection = self.get_collection(child_nft.0)
        //      .ok_or(RmrkError::ChildContractNotApproved)?;

        PSP34Ref::transfer_builder(&child_nft.0, to, child_nft.1, Vec::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;

        Ok(())
    }
}

impl<T> Nesting for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>> + Utils,
{
    /// Add a child NFT (from different collection) to the NFT in this collection
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
    default fn add_child(
        &mut self,
        to_parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        let parent_owner = self.ensure_exists(&to_parent_token_id)?;
        self.accepted(&to_parent_token_id, &child_nft)?;
        self.pending(&to_parent_token_id, &child_nft)?;

        // Transfer child ownership to this contract.
        // This transfer call will fail if caller is not child owner
        self.transfer_child_ownership(Self::env().account_id(), child_nft.clone())?;

        // Insert child nft and emit event
        self._emit_added_child_event(&to_parent_token_id, &child_nft.0, &child_nft.1);
        let caller = Self::env().caller();
        if caller == parent_owner {
            self.add_to_accepted(to_parent_token_id, child_nft);
        } else {
            self.add_to_pending(to_parent_token_id, child_nft);
        }

        Ok(())
    }

    /// Remove a child NFT (from different collection) from token_id in this collection
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
    default fn remove_child(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        self.ensure_exists(&parent_token_id)?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, &parent_token_id)?;

        // Remove child nft
        self.remove_accepted(&parent_token_id, &child_nft)?;

        // Transfer child ownership from this contract to parent_token owner.
        // This call will fail if this contract is not child owner
        let token_owner = self.ensure_exists(&parent_token_id)?;
        self.transfer_child_ownership(token_owner, child_nft)?;

        Ok(())
    }

    /// Accept a child NFT (from different collection) to be owned by parent token
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
    default fn accept_child(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        self.ensure_exists(&parent_token_id)?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, &parent_token_id)?;
        self.accepted(&parent_token_id, &child_nft)?;

        self.remove_from_pending(&parent_token_id, &child_nft)?;
        self.add_to_accepted(parent_token_id, child_nft);

        Ok(())
    }

    /// Reject a child NFT (from different collection)
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
    default fn reject_child(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        self.ensure_exists(&parent_token_id)?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, &parent_token_id)?;
        self.accepted(&parent_token_id, &child_nft)?;

        self.remove_from_pending(&parent_token_id, &child_nft)?;
        self._emit_child_rejected_event(&parent_token_id, &child_nft.0, &child_nft.1);

        Ok(())
    }

    /// Transfer the child NFT from one parent to another (in this collection)
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
    default fn transfer_child(
        &mut self,
        current_parent: Id,
        new_parent: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        let current_parent_owner = self.ensure_exists(&current_parent)?;
        let new_parent_owner = self.ensure_exists(&new_parent)?;
        self.remove_accepted(&current_parent, &child_nft)?;

        self._emit_added_child_event(&new_parent, &child_nft.0, &child_nft.1);
        if current_parent_owner == new_parent_owner {
            self.add_to_accepted(new_parent, child_nft);
        } else {
            self.add_to_pending(new_parent, child_nft);
        }

        Ok(())
    }

    /// Read the number of children on the parent token
    /// # Arguments:
    /// * `parent_token_id`: parent tokenId to check
    ///
    /// # Result:
    /// Returns the tupple of `(accepted_children, pending_children)` count
    fn children_balance(&self, parent_token_id: Id) -> Result<(u64, u64), PSP34Error> {
        self.ensure_exists(&parent_token_id)?;
        let parents_with_accepted_children = match self
            .data::<NestingData>()
            .accepted_children
            .get(&parent_token_id)
        {
            Some(children) => children.len() as u64,
            None => 0,
        };

        let parents_with_pending_children = match self
            .data::<NestingData>()
            .pending_children
            .get(&parent_token_id)
        {
            Some(children) => children.len() as u64,
            None => 0,
        };

        Ok((
            parents_with_accepted_children,
            parents_with_pending_children,
        ))
    }
}

/// Event trait for Nesting
impl<T> NestingEvents for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Emit ChildAdded event
    default fn _emit_added_child_event(
        &self,
        _to: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }
    /// Emit ChildAccepted event
    default fn _emit_child_accepted_event(
        &self,
        _to: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }

    /// Emit ChildRemoved event
    default fn _emit_child_removed_event(
        &self,
        _parent: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }

    /// Emit ChildRejected event
    default fn _emit_child_rejected_event(
        &self,
        _parent: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }
}
