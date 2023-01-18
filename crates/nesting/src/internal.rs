use crate::{
    traits::NestingEvents,
    NestingData,
};

use rmrk_common::{
    errors::RmrkError,
    types::*,
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

/// Trait implementation for Internal Nesting functions.
pub trait Internal {
    /// Check if child is already accepted.
    fn accepted(&self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<(), PSP34Error>;

    /// Check if child is already pending.
    fn pending(&self, parent_token_id: &Id, child_nft: &ChildNft) -> Result<(), PSP34Error>;

    /// Add the child to the list of accepted children.
    fn add_to_accepted(&mut self, parent_token_id: Id, child_nft: ChildNft);

    /// Remove the child to the list of accepted children.
    fn remove_accepted(
        &mut self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error>;

    /// Add the child to the list of pending children.
    fn add_to_pending(&mut self, parent_token_id: Id, child_nft: ChildNft);

    /// Remove the child to the list of pending children.
    fn remove_from_pending(
        &mut self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error>;

    /// Check if caller is the owner of this parent token.
    fn is_caller_parent_owner(
        &self,
        caller: AccountId,
        parent_token_id: &Id,
    ) -> Result<(), PSP34Error>;

    /// Cross contract call to transfer child nft ownership.
    fn transfer_child_ownership(
        &self,
        to: AccountId,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error>;
}

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
