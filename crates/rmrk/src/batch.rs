#![allow(clippy::inline_fn_without_body)]

/// Batch calls for the RMRK contract.
/// Due to big POV size of this contract only a several Tx can be made per one block.
/// This trait allows to batch several calls into one Tx.
/// It is not mandatory to include this trait.
/// However it is possible to include it only if all crates are compiled.
use crate::{
    storage::*,
    traits::*,
};

use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{
        access_control,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
        reentrancy_guard::*,
    },
    traits::{
        AccountId,
        DefaultEnv,
        Storage,
    },
};
use rmrk_common::{
    ensure,
    errors::RmrkError,
    types::*,
};
use rmrk_multiasset::MultiAssetData;
use rmrk_nesting::NestingData;

pub const MAX_BATCH_TOKENS_PER_ASSET: usize = 50;
pub const MAX_BATCH_ADD_CHILDREN: usize = 50;
pub const MAX_BATCH_TOKEN_TRANSFERS: usize = 50;

#[openbrush::wrapper]
pub type BatchCallsRef = dyn BatchCalls;

#[openbrush::trait_definition]
pub trait BatchCalls:
    DefaultEnv
    + Nesting
    + MultiAsset
    + Minting
    + Storage<MultiAssetData>
    + Storage<NestingData>
    + Storage<MintingData>
    + Storage<reentrancy_guard::Data>
    + Storage<metadata::Data>
    + Storage<access_control::Data>
    + Storage<psp34::Data<enumerable::Balances>>
{
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
            _ = MultiAsset::add_asset_to_token(self, token_id.clone(), asset_id, None);
        }
        Ok(())
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
            _ = Nesting::add_child(self, parent_id, (child_contract, child_id));
        }

        Ok(())
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
            _ = Minting::transfer_token(self, destination, token_id, Vec::new());
        }

        Ok(())
    }
}
