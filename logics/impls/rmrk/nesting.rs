// Copyright (c) 2022 Astar Network
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use ink_prelude::collections::BTreeSet;

use crate::impls::rmrk::errors::RmrkError;
use crate::impls::rmrk::types::*;
pub use crate::traits::nesting::{Internal, Nesting, NestingEvents};
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{AccountId, Storage, String},
};

/// Implement internal helper trait for Nesting
impl<T> Internal for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if child is already accepted
    default fn already_accepted(
        &self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self
            .data::<NestingData>()
            .accepted_children
            .get(&parent_token_id)
        {
            if children.contains(&child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedChild.as_str(),
                )));
            }
        }
        Ok(())
    }

    /// Check if child is already pending
    default fn already_pending(
        &self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self
            .data::<NestingData>()
            .pending_children
            .get(&parent_token_id)
        {
            if children.contains(&child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingChild.as_str(),
                )));
            }
        }
        Ok(())
    }

    /// Add the child to the list of accepted children
    default fn add_to_accepted(
        &mut self,
        caller: AccountId,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) {
        self.data::<NestingData>()
            .accepted_children
            .entry(parent_token_id.clone())
            .and_modify(|children| {
                children.insert(child_nft.clone());
            })
            .or_insert_with(|| BTreeSet::from([child_nft.clone()]));
        self._emit_child_accepted_event(caller, parent_token_id, child_nft.0, child_nft.1);
    }

    /// Remove the child to the list of accepted children
    default fn remove_accepted(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self
            .data::<NestingData>()
            .accepted_children
            .get_mut(&parent_token_id.clone())
        {
            if !children.remove(&child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::ChildNotFound.as_str(),
                )));
            }
        }
        Ok(())
    }

    /// Add the child to the list of pending children
    default fn add_to_pending(&mut self, parent_token_id: Id, child_nft: ChildNft) {
        self.data::<NestingData>()
            .pending_children
            .entry(parent_token_id.clone())
            .and_modify(|children| {
                children.insert(child_nft.clone());
            })
            .or_insert_with(|| BTreeSet::from([child_nft.clone()]));
    }

    /// Remove the child to the list of pending children
    default fn remove_from_pending(
        &mut self,
        parent_token_id: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self
            .data::<NestingData>()
            .pending_children
            .get_mut(&parent_token_id.clone())
        {
            if !children.remove(&child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::ChildNotFound.as_str(),
                )));
            }
        }
        Ok(())
    }

    /// Check if token is minted. Return the owner
    default fn ensure_exists(&self, id: Id) -> Result<AccountId, PSP34Error> {
        let token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Check if caller is the owner of this parent token
    default fn is_caller_parent_owner(
        &self,
        caller: AccountId,
        parent_token_id: Id,
    ) -> Result<(), PSP34Error> {
        if let Some(token_owner) = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(parent_token_id.clone())
        {
            if token_owner != caller {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::NotAuthorised.as_str(),
                )));
            }
        }
        Ok(())
    }

    /// Cross contract call to transfer child nft ownership
    default fn transfer_child_ownership(
        &self,
        _signer: AccountId,
        _to: AccountId,
    ) -> Result<(), PSP34Error> {
        todo!()
    }
}

impl<T> Nesting for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Add a child NFT (from different collection) to the NFT to NFT in this collection
    /// The status of added child is `Pending` if caller is not owner of child NFT
    /// The status of added child is `Accepted` if caller is is owner of child NFT
    ///
    /// # Requirements:
    /// * `child_contract_address` needs to be added by collecion owner
    /// * `parent_token_id` must exist.
    /// * `child_token_id` must exist.
    /// * There cannot be two identical children.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Ownership of child NFT will be transferred to this contract (cross contract call)
    /// On success emitts `RmrkEvent::AddedChild`
    /// On success emitts `RmrkEvent::ChildAccepted` - only if caller is already owner of child NFT
    default fn add_child(
        &mut self,
        parent_token_id: ItemId,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        self.ensure_exists(parent_token_id.clone())?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, parent_token_id.clone())?;
        self.already_accepted(parent_token_id.clone(), child_nft.clone())?;
        self.already_pending(parent_token_id.clone(), child_nft.clone())?;

        // TODO check child collection is approved by this (parent) collection

        // TODO send transfer() to child contract
        let child_owner = caller; // TODO

        // Insert child nft and emit event
        self._emit_added_child_event(
            caller,
            parent_token_id.clone(),
            child_nft.0.clone(),
            child_nft.1.clone(),
        );
        if child_owner == caller {
            self.add_to_accepted(caller, parent_token_id.clone(), child_nft.clone());
        } else {
            self.add_to_pending(parent_token_id.clone(), child_nft.clone());
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
        self.ensure_exists(parent_token_id.clone())?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, parent_token_id.clone())?;

        // Remove child nft and emit event
        if let Some(children) = self
            .data::<NestingData>()
            .accepted_children
            .get_mut(&parent_token_id.clone())
        {
            if !children.remove(&child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::ChildNotFound.as_str(),
                )));
            }
        }

        // TODO return ownership of the child nft to parent token owner
        let token_owner = self.ensure_exists(parent_token_id.clone())?;
        self.transfer_child_ownership(Self::env().account_id(), token_owner)?;
        self._emit_child_removed_event(parent_token_id, child_nft.0, child_nft.1);

        Ok(())
    }

    /// Accept a child NFT (from different collection) to be owned by parent token
    ///
    /// # Requirements:
    /// * The status of the child is `Pending`
    /// *
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
        self.ensure_exists(parent_token_id.clone())?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, parent_token_id.clone())?;
        self.already_accepted(parent_token_id.clone(), child_nft.clone())?;

        self.remove_from_pending(parent_token_id.clone(), child_nft.clone())?;
        self.add_to_accepted(caller, parent_token_id, child_nft);

        Ok(())
    }

    /// Reject a child NFT (from different collection)
    ///
    /// # Requirements:
    /// * The status of the child is `Pending`
    /// *
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
        self.ensure_exists(parent_token_id.clone())?;
        let caller = Self::env().caller();
        self.is_caller_parent_owner(caller, parent_token_id.clone())?;
        self.already_accepted(parent_token_id.clone(), child_nft.clone())?;

        self.remove_from_pending(parent_token_id.clone(), child_nft.clone())?;
        self._emit_child_rejected_event(parent_token_id, child_nft.0, child_nft.1);

        Ok(())
    }

    /// Transfer the child NFT from one parent to another (in this collection)
    ///
    /// # Requirements:
    /// * The status of the child is `Accepted`
    /// *
    ///
    /// # Arguments:
    /// * `current_parent`: current parent tokenId which holds child nft
    /// * `new_parent`: new parent tokenId which will hold child nft
    /// * `child_nft`: (collection_id, token_id) of the child instance.
    ///
    /// # Result:
    /// Ownership of child NFT will be transferred to this contract (cross contract call)
    /// On success emitts `RmrkEvent::AddedChild`
    /// On success emitts `RmrkEvent::ChildAccepted` - only if caller is already owner of child NFT
    default fn transfer_child(
        &mut self,
        current_parent: Id,
        new_parent: Id,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        let current_parent_owner = self.ensure_exists(current_parent.clone())?;
        let new_parent_owner = self.ensure_exists(new_parent.clone())?;
        self.remove_accepted(current_parent.clone(), child_nft.clone())?;

        self._emit_added_child_event(
            current_parent_owner,
            new_parent.clone(),
            child_nft.0.clone(),
            child_nft.1.clone(),
        );
        if current_parent_owner == new_parent_owner {
            self.add_to_accepted(new_parent_owner, new_parent.clone(), child_nft.clone());
        } else {
            self.add_to_pending(new_parent.clone(), child_nft.clone());
        }

        Ok(())
    }
}

/// Helper trait for Psp34Custom
impl<T> NestingEvents for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Emit AddedChild event
    default fn _emit_added_child_event(
        &self,
        _from: AccountId,
        _to: Id,
        _child_collection_address: AccountId,
        _child_token_id: Id,
    ) {
    }
    /// Emit ChildAccepted event
    default fn _emit_child_accepted_event(
        &self,
        _approved_by: AccountId,
        _to: Id,
        _child_collection_address: AccountId,
        _child_token_id: Id,
    ) {
    }

    /// Emit ChildRemoved event
    default fn _emit_child_removed_event(
        &self,
        _parent: Id,
        _child_collection_address: AccountId,
        _child_token_id: Id,
    ) {
    }

    /// Emit ChildRejected event
    default fn _emit_child_rejected_event(
        &self,
        _parent: Id,
        _child_collection_address: AccountId,
        _child_token_id: Id,
    ) {
    }
}
