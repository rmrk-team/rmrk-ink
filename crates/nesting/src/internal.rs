use crate::{
    traits::NestingEvents,
    NestingData,
};

use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    types::*,
};

use ink::{
    env::CallFlags,
    prelude::vec::Vec,
};
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Storage,
    },
};

/// Trait implementation for Internal Nesting functions.
pub trait Internal {
    /// Check if child is already accepted.
    fn accepted(&self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<()>;

    /// Check if child is already pending.
    fn pending(&self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<()>;

    /// Add the child to the list of accepted children.
    fn add_to_accepted(&mut self, parent_token_id: Id, child_nft: ChildNft);

    /// Remove the child to the list of accepted children.
    fn remove_accepted(&mut self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<()>;

    /// Add the child to the list of pending children.
    fn add_to_pending(&mut self, parent_token_id: Id, child_nft: ChildNft);

    /// Remove the child to the list of pending children.
    fn remove_from_pending(&mut self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<()>;

    /// Check if caller is the owner of this parent token.
    fn is_caller_parent_owner(&self, caller: AccountId, parent_token_id: &Id) -> Result<()>;

    /// Cross contract call to transfer child nft ownership.
    fn transfer_child_ownership(&self, to: AccountId, child_nft: ChildNft) -> Result<()>;

    /// Set the owner of the provided child nft.
    fn set_parent(&mut self, child_nft: &ChildNft, parent_token_id: Id);

    /// Remove the owner of the provided child nft.
    fn remove_parent(&mut self, child_nft: &ChildNft);
}

/// Implement internal helper trait for Nesting
impl<T> Internal for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if child is already accepted
    default fn accepted(&self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<()> {
        if let Some(children) = self
            .data::<NestingData>()
            .accepted_children
            .get(parent_token_id)
        {
            if children.contains(child_nft) {
                return Err(RmrkError::AlreadyAddedChild.into())
            }
        }
        Ok(())
    }

    /// Check if child is already pending
    default fn pending(&self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<()> {
        if let Some(children) = self
            .data::<NestingData>()
            .pending_children
            .get(parent_token_id)
        {
            if children.contains(child_nft) {
                return Err(RmrkError::AddingPendingChild.into())
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
    ) -> Result<()> {
        let mut child_nfts = self
            .data::<NestingData>()
            .accepted_children
            .get(parent_token_id)
            .ok_or(RmrkError::InvalidParentId)?;

        let index = child_nfts
            .iter()
            .position(|x| x == child_nft)
            .ok_or(RmrkError::ChildNotFound)?;
        child_nfts.remove(index);

        self.data::<NestingData>()
            .accepted_children
            .insert(parent_token_id, &child_nfts);

        self._emit_child_removed_event(parent_token_id, &child_nft.0, &child_nft.1);
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
    ) -> Result<()> {
        let mut child_nfts = self
            .data::<NestingData>()
            .pending_children
            .get(parent_token_id)
            .ok_or(RmrkError::InvalidParentId)?;

        let index = child_nfts
            .iter()
            .position(|x| x == child_nft)
            .ok_or(RmrkError::ChildNotFound)?;
        child_nfts.remove(index);

        self.data::<NestingData>()
            .pending_children
            .insert(parent_token_id, &child_nfts);

        Ok(())
    }

    /// Set the owner of the child nft.
    default fn set_parent(&mut self, child_nft: &ChildNft, parent_token_id: Id) {
        let _ = self
            .data::<NestingData>()
            .parent_of
            .get(child_nft)
            .insert(parent_token_id);
    }

    /// Remove the owner of the child nft.
    default fn remove_parent(&mut self, child_nft: &ChildNft) {
        self.data::<NestingData>().parent_of.take(child_nft);
    }

    /// Check if caller is the owner of this parent token
    default fn is_caller_parent_owner(
        &self,
        caller: AccountId,
        parent_token_id: &Id,
    ) -> Result<()> {
        if let Some(token_owner) = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(parent_token_id.clone())
        {
            if token_owner != caller {
                return Err(RmrkError::NotTokenOwner.into())
            }
        }
        Ok(())
    }

    /// Cross contract call to transfer child nft ownership
    default fn transfer_child_ownership(&self, to: AccountId, child_nft: ChildNft) -> Result<()> {
        // TODO check child collection is approved by this (parent) collection
        // let collection = self.get_collection(child_nft.0)
        //      .ok_or(RmrkError::ChildContractNotApproved)?;

        PSP34Ref::transfer_builder(&child_nft.0, to, child_nft.1, Vec::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke()
            .unwrap();

        Ok(())
    }
}
