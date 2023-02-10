//! RMRK Base implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod internal;
pub mod traits;

use internal::Internal;

use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    types::*,
    utils::Utils,
};

use traits::{
    MultiAsset,
    MultiAssetEvents,
};

use ink_prelude::vec::Vec;
use ink_storage::Mapping;

use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::enumerable::*,
    },
    modifiers,
    traits::{
        Storage,
        String,
    },
};

pub const STORAGE_MULTIASSET_KEY: u32 = openbrush::storage_unique_key!(MultiAssetData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MULTIASSET_KEY)]
pub struct MultiAssetData {
    /// Mapping of available asset entries for this collection
    pub collection_asset_entries: Mapping<AssetId, Asset>,

    /// Collection asset id list
    pub collection_asset_ids: Vec<AssetId>,

    /// Mapping of tokenId to an array of active assets
    pub accepted_assets: Mapping<Id, Vec<AssetId>>,

    /// Mapping of tokenId to an array of pending assets
    pub pending_assets: Mapping<Id, Vec<AssetId>>,
}

impl<T> MultiAsset for T
where
    T: Storage<MultiAssetData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<ownable::Data>
        + Utils,
{
    /// Used to add a asset entry.
    #[modifiers(only_owner)]
    fn add_asset_entry(
        &mut self,
        asset_id: AssetId,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<()> {
        self.ensure_asset_id_is_available(asset_id)?;
        self.data::<MultiAssetData>()
            .collection_asset_entries
            .insert(
                asset_id,
                &Asset {
                    equippable_group_id,
                    asset_uri,
                    part_ids: part_ids.clone(),
                },
            );
        self.data::<MultiAssetData>()
            .collection_asset_ids
            .push(asset_id);
        self._emit_asset_set_event(&asset_id);

        Ok(())
    }

    /// Used to add an asset to a token.
    /// tokenId - ID of the token to add the asset to
    /// assetId - ID of the asset to add to the token
    /// replacesAssetWithId - ID of the asset to replace from the token's list of active assets
    fn add_asset_to_token(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        replaces_asset_with_id: Option<AssetId>,
    ) -> Result<()> {
        // Check if asset id is valid
        self.data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
            .ok_or(RmrkError::AssetIdNotFound)?;
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        self.ensure_not_accepted(&token_id, &asset_id)?;
        self.ensure_not_pending(&token_id, &asset_id)?;
        self._emit_asset_added_to_token_event(&token_id, &asset_id, &replaces_asset_with_id);

        if let Some(replace_with_id) = replaces_asset_with_id {
            ink_env::debug_println!("replaces_asset_with_id {:?}", &replaces_asset_with_id);
            return self.replace_asset(&token_id, &asset_id, &replace_with_id)
        } else {
            let caller = Self::env().caller();
            // If the asset is being added by the current root owner of the token, the asset will be automatically accepted.
            if caller == token_owner {
                self.add_to_accepted_assets(&token_id, &asset_id);
            } else {
                self.add_to_pending_assets(&token_id, &asset_id);
            }
        }

        Ok(())
    }

    /// Accepts an asset from the pending array of given token.
    fn accept_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<()> {
        self.ensure_pending(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        let caller = Self::env().caller();
        if caller == token_owner {
            self.remove_from_pending_assets(&token_id, &asset_id)?;
            self.add_to_accepted_assets(&token_id, &asset_id);
        } else {
            return Err(RmrkError::NotTokenOwner.into())
        }
        Ok(())
    }

    /// Rejects an asset from the pending array of given token.
    fn reject_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<()> {
        self.ensure_pending(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        self.ensure_token_owner(token_owner)?;

        self.remove_from_pending_assets(&token_id, &asset_id)?;

        self._emit_asset_rejected_event(&token_id, &asset_id);
        Ok(())
    }

    /// Remove an asset from the pending array of given token.
    fn remove_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<()> {
        self.ensure_asset_accepted(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        self.ensure_token_owner(token_owner)?;

        self.remove_from_accepted_assets(&token_id, &asset_id)?;

        self._emit_asset_removed_event(&token_id, &asset_id);
        Ok(())
    }

    /// Used to specify the priorities for a given token's active assets.
    fn set_priority(&mut self, token_id: Id, priorities: Vec<AssetId>) -> Result<()> {
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        if let Some(accepted_assets) = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(token_id.clone())
        {
            if accepted_assets.len() != priorities.len() {
                return Err(RmrkError::BadPriorityLength.into())
            }
            for asset in priorities.clone() {
                if !accepted_assets.contains(&asset) {
                    return Err(RmrkError::AssetIdNotFound.into())
                }
            }
        }

        self.data::<MultiAssetData>()
            .accepted_assets
            .insert(&token_id, &priorities);
        self._emit_asset_priority_set_event(&token_id, priorities);
        Ok(())
    }

    /// Used to retrieve the total number of asset entries
    fn total_assets(&self) -> u32 {
        self.data::<MultiAssetData>().collection_asset_ids.len() as u32
    }

    /// Used to retrieve the total number of assets per token
    fn total_token_assets(&self, token_id: Id) -> Result<(u64, u64)> {
        self.ensure_exists_and_get_owner(&token_id)?;

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

    /// Check that asset id does not already exist.
    default fn ensure_asset_id_is_available(&self, asset_id: AssetId) -> Result<()> {
        if self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
            .is_some()
        {
            return Err(RmrkError::AssetIdAlreadyExists.into())
        }
        return Ok(())
    }

    /// Used to retrieve asset's uri
    default fn get_asset_uri(&self, asset_id: AssetId) -> Option<String> {
        if let Some(asset) = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
        {
            return Some(asset.asset_uri)
        }
        return None
    }

    /// Fetch all accepted assets for the token_id
    fn get_accepted_token_assets(&self, token_id: Id) -> Result<Option<Vec<AssetId>>> {
        self.ensure_exists_and_get_owner(&token_id)?;
        Ok(self.data::<MultiAssetData>().accepted_assets.get(&token_id))
    }
}

/// Event trait for MultiAssets
impl<T> MultiAssetEvents for T
where
    T: Storage<MultiAssetData>,
{
    /// Used to notify listeners that an asset object is initialized at `assetId`.
    default fn _emit_asset_set_event(&self, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
    default fn _emit_asset_added_to_token_event(
        &self,
        _token_id: &Id,
        _asset_id: &AssetId,
        _replaces_id: &Option<AssetId>,
    ) {
    }

    /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
    default fn _emit_asset_accepted_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped from the pending assets array of the token.
    default fn _emit_asset_rejected_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is removed from token
    default fn _emit_asset_removed_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that token's prioritiy array is reordered.
    default fn _emit_asset_priority_set_event(&self, _token_id: &Id, _priorities: Vec<AssetId>) {}
}
