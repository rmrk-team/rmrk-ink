//! This module enables multiasset capability of RMRK
use crate::impls::rmrk::{
    errors::RmrkError,
    types::*,
};
pub use crate::traits::multiasset::{
    Internal,
    MultiAsset,
    MultiAssetEvents,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
    },
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Implement internal helper trait for MultiAsset
impl<T> Internal for T
where
    T: Storage<MultiAssetData> + Storage<psp34::Data<enumerable::Balances>>,
{
    fn resource_add(
        &mut self,
        _parent_token_id: Id,
        _child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        todo!()
    }
}

impl<T> MultiAsset for T
where
    T: Storage<MultiAssetData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<ownable::Data>,
{
    /// Used to add a asset entry.
    #[modifiers(only_owner)]
    fn add_asset_entry(&mut self, asset_uri: String) -> Result<(), PSP34Error> {
        self.data::<MultiAssetData>()
            .collection_asset_entries
            .push(asset_uri);
        Ok(())
    }

    /// Used to add an asset to a token.
    fn add_asset_to_token(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        replaces_asset_with_id: Id,
    ) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Accepts an asset at from the pending array of given token.

    fn accept_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Rejects an asset from the pending array of given token.
    fn reject_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Used to specify the priorities for a given token's active assets.
    fn set_priority(&mut self, token_id: Id, priorities: Vec<AssetId>) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Used to retrieve the total number of assets.
    fn total_assets(&self) -> u32 {
        self.data::<MultiAssetData>().collection_asset_entries.len() as u32
    }
}
