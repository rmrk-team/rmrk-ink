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

use ink_prelude::collections::BTreeMap;

use crate::impls::rmrk::errors::RmrkError;
use crate::impls::rmrk::types::*;
pub use crate::traits::nesting::Nesting;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{enumerable::*, metadata::*},
        reentrancy_guard::*,
    },
    traits::{AccountId, Storage},
};

pub trait NestingEvents {
    /// Emit AddedChild event
    fn _emit_added_child_event(
        &self,
        from: AccountId,
        to: Id,
        child_collection_address: AccountId,
        child_token_id: Id,
    );
    /// Emit ChildAccepted event
    fn _emit_child_accepted_event(
        &self,
        by: AccountId,
        to: Id,
        child_collection_address: AccountId,
        child_token_id: Id,
    );
}

impl<T> Nesting for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>, // + Storage<reentrancy_guard::Data>
                                                                          // + Storage<ownable::Data>
                                                                          // + Storage<metadata::Data>
                                                                          // + psp34::extensions::metadata::PSP34Metadata,
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
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// # Result:
    /// On success emitts `RmrkEvent::PendingChild` - if caller is not owner of child NFT
    /// On success emitts `RmrkEvent::AcceptedChild` - if caller is already owner of child NFT
    default fn add_child(
        &mut self,
        parent_token_id: ItemId,
        child_nft: Nft,
    ) -> Result<(), PSP34Error> {
        // TODO check child token exists

        // TODO check parent token exists

        // TODO check child collection is approved by this (parent) collection

        // TODO check ownerOf child with xContract call
        let caller = Self::env().caller();
        let child_owner = caller; // TODO

        let parent_token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(parent_token_id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;

        // TODO send transfer() to child contract

        // insert child nft and emit event
        if child_owner == parent_token_owner {
            self.data::<NestingData>()
                .accepted_children
                .insert(parent_token_id.clone(), child_nft.clone());
            self._emit_added_child_event(
                caller,
                parent_token_id.clone(),
                child_nft.0.clone(),
                child_nft.1.clone(),
            );
            self._emit_child_accepted_event(caller, parent_token_id, child_nft.0, child_nft.1);
        } else {
            self.data::<NestingData>()
                .pending_children
                .insert(parent_token_id.clone(), child_nft.clone());
            self._emit_added_child_event(caller, parent_token_id, child_nft.0, child_nft.1);
        }

        Ok(())
    }

    default fn remove_child(
        &mut self,
        parent_token_id: Id,
        child_contract_address: AccountId,
        child_token_id: Id,
    ) -> Result<(), PSP34Error> {
        todo!()
    }
    default fn accept_child(
        &mut self,
        parent_token_id: Id,
        child_contract_address: AccountId,
        child_token_id: Id,
    ) -> Result<(), PSP34Error> {
        todo!()
    }
    default fn reject_child(
        &mut self,
        parent_token_id: Id,
        child_contract_address: AccountId,
        child_token_id: Id,
    ) -> Result<(), PSP34Error> {
        todo!()
    }
    default fn transfer_child(
        &mut self,
        parent_token_id: Id,
        child_contract_address: AccountId,
        child_token_id: Id,
        to: AccountId,
    ) -> Result<(), PSP34Error> {
        todo!()
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
    /// Emit AddedChild event
    default fn _emit_child_accepted_event(
        &self,
        _approved_by: AccountId,
        _to: Id,
        _child_collection_address: AccountId,
        _child_token_id: Id,
    ) {
    }
}
