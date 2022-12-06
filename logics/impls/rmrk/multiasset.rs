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
    /// Check if token is minted. Return the token uri
    fn asset_id_exists(&self, asset_id: AssetId) -> Option<String> {
        if let Some(index) = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .iter()
            .position(|a| a.asset_id == asset_id)
        {
            let asset_uri =
                &self.data::<MultiAssetData>().collection_asset_entries[index].asset_uri;
            return Some(asset_uri.clone())
        }

        None
    }

    /// Check if token is minted. Return the owner
    fn ensure_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        let token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Check if asset is already accepted
    default fn is_accepted(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error> {
        if let Some(children) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if children.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn is_pending(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Add the child to the list of accepted children
    default fn add_to_accepted_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(asset_id.clone());
            self.data::<MultiAssetData>()
                .accepted_assets
                .insert(&token_id, &assets);
        }
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
    fn add_asset_entry(
        &mut self,
        asset_id: AssetId,
        equippable_group_id: EquippableGroupId,
        base_id: BaseId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<(), PSP34Error> {
        if self.asset_id_exists(asset_id).is_some() {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::AssetIdAlreadyExists.as_str(),
            )))
        };
        self.data::<MultiAssetData>()
            .collection_asset_entries
            .push(Asset {
                asset_id,
                equippable_group_id,
                base_id,
                asset_uri,
                part_ids,
            });
        Ok(())
    }

    /// Used to add an asset to a token.
    #[modifiers(only_owner)]
    fn add_asset_to_token(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        _replaces_asset_with_id: Option<Id>,
    ) -> Result<(), PSP34Error> {
        _ = self
            .asset_id_exists(asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::AssetIdNotFound.as_str(),
            )));
        self.ensure_exists(&token_id)?;
        self.is_accepted(&token_id, &asset_id)?;
        self.is_pending(&token_id, &asset_id)?;
        self.add_to_accepted_assets(&token_id, &asset_id);
        self._emit_asset_added_to_token_event(&token_id, &asset_id, None);

        Ok(())
    }

    /// Accepts an asset at from the pending array of given token.
    fn accept_asset(&mut self, _token_id: Id, _asset_id: AssetId) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Rejects an asset from the pending array of given token.
    fn reject_asset(&mut self, _token_id: Id, _asset_id: AssetId) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Used to specify the priorities for a given token's active assets.
    fn set_priority(&mut self, token_id: Id, priorities: Vec<AssetId>) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Used to retrieve the total number of asset entries
    fn total_assets(&self) -> u32 {
        self.data::<MultiAssetData>().collection_asset_entries.len() as u32
    }

    /// Used to retrieve the total number of assets per token
    fn total_token_assets(&self, token_id: Id) -> Result<(u64, u64), PSP34Error> {
        self.ensure_exists(&token_id)?;
        let accepted_assets_on_token =
            match self.data::<MultiAssetData>().accepted_assets.get(&token_id) {
                Some(assets) => assets.len() as u64,
                None => 0,
            };

        let pending_assets_on_token =
            match self.data::<MultiAssetData>().pending_assets.get(&token_id) {
                Some(assets) => assets.len() as u64,
                None => 0,
            };

        Ok((accepted_assets_on_token, pending_assets_on_token))
    }

    /// Used to retrieve asset's uri
    fn get_asset_uri(&self, asset_id: AssetId) -> Option<String> {
        self.asset_id_exists(asset_id)
    }
}

/// Event trait for MultiAssets
impl<T> MultiAssetEvents for T
where
    T: Storage<MultiAssetData>,
{
    /// Used to notify listeners that an asset object is initialized at `assetId`.
    fn _emit_asset_set_event(&self, asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
    fn _emit_asset_added_to_token_event(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
        replaces_id: Option<Id>,
    ) {
    }

    /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
    fn _emit_asset_accepted_event(&self, token_id: &Id, asset_id: &Id, replaces_id: &Id) {}

    /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped from the pending assets array of the token.
    fn _emit_asset_rejected_event(&self, token_id: &Id, asset_id: &Id) {}

    /// Used to notify listeners that token's prioritiy array is reordered.
    fn _emit_asset_priority_set_event(&self, token_id: &Id) {}
}
