#![allow(clippy::inline_fn_without_body)]

use crate::traits::{
    MintingRef,
    MultiAssetRef,
    NestingRef,
};

use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        DefaultEnv,
    },
};
use rmrk_common::{
    ensure,
    errors::RmrkError,
    types::*,
};

pub const MAX_BATCH_TOKENS_PER_ASSET: usize = 50;
pub const MAX_BATCH_ADD_CHILDREN: usize = 50;
pub const MAX_BATCH_TOKEN_TRANSFERS: usize = 50;

#[openbrush::wrapper]
pub type BatchCallsRef = dyn BatchCalls;

#[openbrush::trait_definition]
pub trait BatchCalls: DefaultEnv {
    #[ink(message)]
    fn add_single_asset_to_token(&mut self, token_id: Id, asset_id: AssetId) {
        _ = MultiAssetRef::add_asset_to_token_builder(
            &<Self as DefaultEnv>::env().account_id(),
            token_id,
            asset_id,
            None,
        )
        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
        .try_invoke();
    }

    #[ink(message)]
    fn add_asset_to_many_tokens(
        &mut self,
        tokens: Vec<Id>,
        asset_id: AssetId,
    ) -> Result<(), RmrkError> {
        ensure!(
            tokens.len() <= MAX_BATCH_TOKENS_PER_ASSET,
            RmrkError::InputVectorTooBig
        );
        for token_id in tokens {
            self.add_single_asset_to_token(token_id, asset_id);
        }
        Ok(())
    }

    /// Add the child NFT.
    #[ink(message)]
    fn add_single_child(&mut self, parent_id: Id, child_contract: AccountId, child_id: Id) {
        _ = NestingRef::add_child_builder(
            &<Self as DefaultEnv>::env().account_id(),
            parent_id,
            (child_contract, child_id),
        )
        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
        .try_invoke();
    }

    /// Add a list of parent-child token pairs. The child NFT is from a different collection.
    #[ink(message)]
    fn add_many_children(
        &mut self,
        child_contract: AccountId,
        parent_child_pair: Vec<(Id, Id)>,
    ) -> Result<(), RmrkError> {
        ensure!(
            parent_child_pair.len() <= MAX_BATCH_ADD_CHILDREN,
            RmrkError::InputVectorTooBig
        );
        for (parent_id, child_id) in parent_child_pair {
            self.add_single_child(parent_id, child_contract, child_id);
        }

        Ok(())
    }

    /// Transfer a single token to specified address
    #[ink(message)]
    fn transfer_single_token(&mut self, destination: AccountId, token_id: Id) {
        _ = MintingRef::transfer_token_builder(
            &<Self as DefaultEnv>::env().account_id(),
            destination,
            token_id,
            Vec::new(),
        )
        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
        .try_invoke();
    }

    /// Transfer many tokens to specified addresses
    #[ink(message)]
    fn transfer_many(
        &mut self,
        token_to_destination: Vec<(Id, AccountId)>,
    ) -> Result<(), RmrkError> {
        ensure!(
            token_to_destination.len() <= MAX_BATCH_TOKEN_TRANSFERS,
            RmrkError::InputVectorTooBig
        );
        for (token_id, destination) in token_to_destination {
            self.transfer_single_token(destination, token_id);
        }

        Ok(())
    }
}
